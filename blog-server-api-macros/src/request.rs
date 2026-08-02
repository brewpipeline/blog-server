use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, Path, parse_macro_input};

enum Source {
    Extension,
    Data,
    Path(String),
    Query(String),
    Header(String),
}

fn inner_of(ty: &syn::Type, wrapper: &str) -> Option<syn::Type> {
    let syn::Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    if segment.ident != wrapper {
        return None;
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    args.args.iter().find_map(|arg| match arg {
        syn::GenericArgument::Type(inner) => Some(inner.clone()),
        _ => None,
    })
}

fn is_string(ty: &syn::Type) -> bool {
    matches!(ty, syn::Type::Path(path)
        if path.path.segments.last().is_some_and(|s| s.ident == "String"))
}

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let Data::Struct(data) = input.data else {
        return syn::Error::new_spanned(name, "ApiRequest only applies to structs")
            .to_compile_error()
            .into();
    };
    let fields = match data.fields {
        Fields::Named(fields) => fields.named,
        Fields::Unit => Default::default(),
        Fields::Unnamed(_) => {
            return syn::Error::new_spanned(name, "ApiRequest needs named fields")
                .to_compile_error()
                .into();
        }
    };

    let mut declared_failure: Option<Path> = None;
    for attr in &input.attrs {
        if !attr.path().is_ident("request") {
            continue;
        }
        let parsed = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("failure") {
                declared_failure = Some(meta.value()?.parse()?);
                Ok(())
            } else {
                Err(meta.error("the only struct level option is `failure = ...`"))
            }
        });
        if let Err(error) = parsed {
            return error.to_compile_error().into();
        }
    }

    let mut assignments = Vec::new();
    let mut bounds = Vec::new();
    let mut data_type = None;
    let mut fallible = false;

    for field in &fields {
        let field_name = field.ident.as_ref().unwrap();
        let mut source = None;
        let mut fallbacks: Vec<String> = Vec::new();
        let mut default: Option<String> = None;

        for attr in &field.attrs {
            if !attr.path().is_ident("request") {
                continue;
            }
            let parsed = attr.parse_nested_meta(|meta| {
                let key = meta
                    .path
                    .get_ident()
                    .ok_or_else(|| meta.error("expected an identifier"))?
                    .to_string();
                match key.as_str() {
                    "extension" => source = Some(Source::Extension),
                    "data" => source = Some(Source::Data),
                    "path" => source = Some(Source::Path(meta.value()?.parse::<LitStr>()?.value())),
                    "query" => {
                        source = Some(Source::Query(meta.value()?.parse::<LitStr>()?.value()))
                    }
                    "header" => {
                        source = Some(Source::Header(meta.value()?.parse::<LitStr>()?.value()))
                    }
                    "or" => fallbacks.push(meta.value()?.parse::<LitStr>()?.value()),
                    "default" => default = Some(meta.value()?.parse::<LitStr>()?.value()),
                    other => return Err(meta.error(format!("unknown request option `{other}`"))),
                }
                Ok(())
            });
            if let Err(error) = parsed {
                return error.to_compile_error().into();
            }
        }

        let Some(source) = source else {
            return syn::Error::new_spanned(
                field,
                "every field needs #[request(extension)], #[request(data)], \
                 #[request(path = \"...\")] or #[request(query = \"...\")]",
            )
            .to_compile_error()
            .into();
        };

        if !matches!(source, Source::Header(_)) && (!fallbacks.is_empty() || default.is_some()) {
            return syn::Error::new_spanned(
                field,
                "`or` and `default` only mean something for a `header` field",
            )
            .to_compile_error()
            .into();
        }

        let ty = &field.ty;
        let expression = match source {
            Source::Extension => {
                bounds.push(quote! { crate::extensions::Resolve<#ty> });
                quote! { origin_content.extensions.resolve() }
            }
            Source::Data => match inner_of(ty, "DResult") {
                Some(inner) => {
                    data_type = Some(inner);
                    quote! { origin_content.data_result }
                }
                None => {
                    data_type = Some(ty.clone());
                    fallible = true;
                    quote! {
                        origin_content
                            .data_result
                            .map_err(crate::utils::body_rejection::BodyRejection)?
                    }
                }
            },
            Source::Path(key) => {
                let segment = quote! {
                    origin_content.path.get(#key).map(|n| n.to_owned()).unwrap_or_default()
                };
                if inner_of(ty, "Option").is_some() {
                    quote! { origin_content.path.get(#key).map(|n| n.to_owned()) }
                } else if is_string(ty) {
                    segment
                } else {
                    fallible = true;
                    quote! { #segment.parse()? }
                }
            }
            Source::Query(key) => {
                let inner = inner_of(ty, "Option");
                match inner {
                    Some(inner) if is_string(&inner) => {
                        quote! { origin_content.query.get(#key).map(|n| n.to_owned()) }
                    }
                    Some(_) => quote! {
                        origin_content.query.get(#key).and_then(|v| v.parse().ok())
                    },
                    None => {
                        return syn::Error::new_spanned(
                            field,
                            "a query field must be an `Option<T>`",
                        )
                        .to_compile_error()
                        .into();
                    }
                }
            }
            Source::Header(name) => {
                let names = std::iter::once(&name).chain(fallbacks.iter());
                let value = quote! {
                    None #( .or_else(|| origin_content.http_parts.headers.get(#names)) )*
                        .and_then(|value| value.to_str().ok())
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                };
                match (inner_of(ty, "Option"), default) {
                    (Some(inner), _) if is_string(&inner) => {
                        quote! { #value.map(str::to_owned) }
                    }
                    (Some(_), _) => {
                        return syn::Error::new_spanned(
                            field,
                            "an optional header field must be an `Option<String>`; a parsed \
                             header is required, so that a value nobody can read is refused \
                             rather than silently dropped",
                        )
                        .to_compile_error()
                        .into();
                    }
                    (None, Some(default)) if is_string(ty) => {
                        quote! { #value.unwrap_or(#default).to_owned() }
                    }
                    (None, Some(_)) => {
                        return syn::Error::new_spanned(
                            field,
                            "`default` only means something for a `String` header",
                        )
                        .to_compile_error()
                        .into();
                    }
                    (None, None) => {
                        fallible = true;
                        let missing = quote! {
                            #value.ok_or_else(|| {
                                crate::utils::header_rejection::HeaderRejection::missing(#name)
                            })?
                        };
                        if is_string(ty) {
                            quote! { #missing.to_owned() }
                        } else {
                            quote! {
                                #missing.parse().map_err(|error| {
                                    crate::utils::header_rejection::HeaderRejection::unparsed(
                                        #name, error,
                                    )
                                })?
                            }
                        }
                    }
                }
            }
        };
        assignments.push(quote! { #field_name: #expression });
    }

    let data_type = data_type.map(|ty| quote! { #ty }).unwrap_or(quote! { () });

    let (generics, failure, mut predicates) = match (fallible, declared_failure) {
        (true, Some(failure)) => (quote! { <Extensions> }, quote! { #failure }, Vec::new()),
        (true, None) => {
            return syn::Error::new_spanned(
                name,
                "a content with a parsed path field needs #[request(failure = ...)] naming the \
                 failure type its endpoint answers with",
            )
            .to_compile_error()
            .into();
        }
        (false, Some(failure)) => (quote! { <Extensions> }, quote! { #failure }, Vec::new()),
        (false, None) => (
            quote! { <Extensions, Failure> },
            quote! { Failure },
            vec![quote! { Failure: screw_api::response::ApiResponseContentFailure }],
        ),
    };
    predicates.push(quote! { Extensions: Send + Sync #(+ #bounds)* });
    let where_clause = (!predicates.is_empty()).then(|| quote! { where #(#predicates),* });

    quote! {
        impl #generics screw_api::request::ApiRequestContent<Extensions, #failure> for #name
        #where_clause
        {
            type Data = #data_type;

            async fn create(
                origin_content: screw_api::request::ApiRequestOriginContent<Self::Data, Extensions>,
            ) -> Result<Self, #failure> {
                Ok(Self { #(#assignments),* })
            }
        }
    }
    .into()
}
