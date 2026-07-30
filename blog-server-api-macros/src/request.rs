use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, parse_macro_input};

enum Source {
    Extension,
    Data,
    Path(String),
    Query(String),
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

    let mut assignments = Vec::new();
    let mut bounds = Vec::new();
    let mut data_type = None;

    for field in &fields {
        let field_name = field.ident.as_ref().unwrap();
        let mut source = None;

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

        let ty = &field.ty;
        let expression = match source {
            Source::Extension => {
                bounds.push(quote! { crate::extensions::Resolve<#ty> });
                quote! { origin_content.extensions.resolve() }
            }
            Source::Data => {
                let Some(inner) = inner_of(ty, "DResult") else {
                    return syn::Error::new_spanned(field, "a data field must be a `DResult<T>`")
                        .to_compile_error()
                        .into();
                };
                data_type = Some(inner);
                quote! { origin_content.data_result }
            }
            Source::Path(key) => match inner_of(ty, "Option") {
                Some(_) => quote! { origin_content.path.get(#key).map(|n| n.to_owned()) },
                None => quote! {
                    origin_content.path.get(#key).map(|n| n.to_owned()).unwrap_or_default()
                },
            },
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
        };
        assignments.push(quote! { #field_name: #expression });
    }

    let data_type = data_type.map(|ty| quote! { #ty }).unwrap_or(quote! { () });
    let where_clause = (!bounds.is_empty()).then(|| quote! { where Extensions: #(#bounds)+* });

    quote! {
        impl<Extensions> screw_api::request::ApiRequestContent<Extensions> for #name
        #where_clause
        {
            type Data = #data_type;

            fn create(
                origin_content: screw_api::request::ApiRequestOriginContent<Self::Data, Extensions>,
            ) -> Self {
                Self { #(#assignments),* }
            }
        }
    }
    .into()
}
