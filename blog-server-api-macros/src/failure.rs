use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident, LitStr, Variant, parse_macro_input};

struct Spec {
    auth: bool,
    database: bool,
    incorrect_id: bool,
    body: bool,
    status: Option<Ident>,
    reason: Option<String>,
    debug_reason: Option<String>,
}

impl Spec {
    fn empty() -> Self {
        Self {
            auth: false,
            database: false,
            incorrect_id: false,
            body: false,
            status: None,
            reason: None,
            debug_reason: None,
        }
    }

    fn sugar(&mut self, status: &str, reason: &str, debug_reason: Option<&str>) {
        self.status = Some(Ident::new(status, proc_macro2::Span::call_site()));
        self.reason = Some(reason.to_owned());
        self.debug_reason = debug_reason.map(str::to_owned);
    }
}

fn upper_snake(name: &str) -> String {
    let mut out = String::new();
    for (index, character) in name.char_indices() {
        if character.is_uppercase() && index != 0 {
            out.push('_');
        }
        out.extend(character.to_uppercase());
    }
    out
}

fn single_named_field(variant: &Variant) -> syn::Result<Ident> {
    let Fields::Named(named) = &variant.fields else {
        return Err(syn::Error::new_spanned(
            variant,
            "this shorthand needs a struct variant with one named field holding the reason",
        ));
    };
    let mut fields = named.named.iter();
    match (fields.next(), fields.next()) {
        (Some(field), None) => Ok(field.ident.clone().unwrap()),
        _ => Err(syn::Error::new_spanned(
            variant,
            "this shorthand needs exactly one named field",
        )),
    }
}

fn parse_spec(variant: &Variant) -> syn::Result<Spec> {
    let mut spec = Spec::empty();
    let mut found = false;

    for attr in &variant.attrs {
        if !attr.path().is_ident("failure") {
            continue;
        }
        found = true;
        attr.parse_nested_meta(|meta| {
            let key = meta
                .path
                .get_ident()
                .ok_or_else(|| meta.error("expected an identifier"))?
                .to_string();
            match key.as_str() {
                "auth" => spec.auth = true,
                "database" => {
                    spec.database = true;
                    spec.sugar(
                        "INTERNAL_SERVER_ERROR",
                        "internal database error",
                        Some("database error: {reason}"),
                    )
                }
                "validation" => {
                    spec.body = true;
                    spec.sugar("BAD_REQUEST", "validation error: {reason}", None)
                }
                "params" => {
                    spec.body = true;
                    spec.sugar("BAD_REQUEST", "params error: {reason}", None)
                }
                "token" => spec.sugar(
                    "INTERNAL_SERVER_ERROR",
                    "internal token generating error",
                    Some("token generating error: {reason}"),
                ),
                "not_found" => {
                    let entity: LitStr = meta.value()?.parse()?;
                    spec.sugar(
                        "NOT_FOUND",
                        &format!("{} record not found in database", entity.value()),
                        None,
                    );
                }
                "incorrect_id" => {
                    let entity: LitStr = meta.value()?.parse()?;
                    spec.incorrect_id = true;
                    spec.sugar(
                        "BAD_REQUEST",
                        &format!(
                            "incorrect value provided for {} ID: {{reason}}",
                            entity.value()
                        ),
                        None,
                    );
                }
                "status" => spec.status = Some(meta.value()?.parse()?),
                "reason" => spec.reason = Some(meta.value()?.parse::<LitStr>()?.value()),
                "debug_reason" => {
                    spec.debug_reason = Some(meta.value()?.parse::<LitStr>()?.value())
                }
                other => return Err(meta.error(format!("unknown failure option `{other}`"))),
            }
            Ok(())
        })?;
    }

    if !found {
        return Err(syn::Error::new_spanned(
            variant,
            "every variant needs a #[failure(...)] attribute",
        ));
    }
    if !spec.auth && spec.status.is_none() {
        return Err(syn::Error::new_spanned(
            variant,
            "a non-auth variant needs a status, either directly or through a shorthand",
        ));
    }
    Ok(spec)
}

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let Data::Enum(data) = input.data else {
        return syn::Error::new_spanned(name, "ApiFailure only applies to enums")
            .to_compile_error()
            .into();
    };

    let mut status_arms = Vec::new();
    let mut identifier_arms = Vec::new();
    let mut reason_arms = Vec::new();
    let mut auth_variant = None;
    let mut database_variant: Option<(Ident, Ident)> = None;
    let mut incorrect_id_variant: Option<(Ident, Ident)> = None;
    let mut body_variant: Option<(Ident, Ident)> = None;

    for variant in &data.variants {
        let spec = match parse_spec(variant) {
            Ok(spec) => spec,
            Err(error) => return error.to_compile_error().into(),
        };
        let variant_name = &variant.ident;

        if spec.auth {
            auth_variant = Some(variant_name.clone());
            status_arms.push(quote! { Self::#variant_name(inner) => inner.status_code() });
            identifier_arms.push(quote! { Self::#variant_name(inner) => inner.identifier() });
            reason_arms.push(quote! { Self::#variant_name(inner) => inner.reason() });
            continue;
        }

        let status = spec.status.unwrap();
        let identifier = upper_snake(&variant_name.to_string());
        let ignore = match variant.fields {
            Fields::Unit => quote! {},
            _ => quote! { { .. } },
        };
        status_arms.push(quote! {
            Self::#variant_name #ignore => hyper::StatusCode::#status
        });
        identifier_arms.push(quote! {
            Self::#variant_name #ignore => #identifier
        });

        if spec.database || spec.incorrect_id || spec.body {
            let field_name = match single_named_field(variant) {
                Ok(field_name) => field_name,
                Err(error) => return error.to_compile_error().into(),
            };
            let (slot, shorthand) = if spec.database {
                (&mut database_variant, "`database`")
            } else if spec.incorrect_id {
                (&mut incorrect_id_variant, "`incorrect_id`")
            } else {
                (&mut body_variant, "`validation` and `params`")
            };
            if let Some((existing, _)) = slot {
                return syn::Error::new_spanned(
                    variant,
                    format!(
                        "{shorthand} generates one conversion for the whole enum, so only one \
                         variant may claim it; `{existing}` already did"
                    ),
                )
                .to_compile_error()
                .into();
            }
            *slot = Some((variant_name.clone(), field_name));
        }

        let bindings = match &variant.fields {
            Fields::Unit => quote! {},
            Fields::Named(named) => {
                let idents = named.named.iter().map(|field| &field.ident);
                quote! { { #(#idents),* } }
            }
            Fields::Unnamed(_) => {
                return syn::Error::new_spanned(
                    variant,
                    "tuple variants are only supported for the auth variant",
                )
                .to_compile_error()
                .into();
            }
        };
        let reason = spec.reason.unwrap_or_default();
        let body = match spec.debug_reason {
            Some(debug_reason) => quote! {
                Some(if cfg!(debug_assertions) {
                    format!(#debug_reason)
                } else {
                    format!(#reason)
                })
            },
            None => quote! { Some(format!(#reason)) },
        };
        reason_arms.push(quote! { Self::#variant_name #bindings => #body });
    }

    let from_auth = auth_variant.map(|variant_name| {
        quote! {
            impl From<crate::utils::auth_middleware::AuthRejection> for #name {
                fn from(value: crate::utils::auth_middleware::AuthRejection) -> Self {
                    Self::#variant_name(value)
                }
            }
        }
    });

    let from_database = database_variant.map(|(variant_name, field_name)| {
        quote! {
            impl From<screw_components::dyn_result::DError> for #name {
                fn from(value: screw_components::dyn_result::DError) -> Self {
                    Self::#variant_name { #field_name: value.to_string() }
                }
            }
        }
    });

    let from_body = body_variant.map(|(variant_name, field_name)| {
        quote! {
            impl From<crate::utils::body_rejection::BodyRejection> for #name {
                fn from(value: crate::utils::body_rejection::BodyRejection) -> Self {
                    Self::#variant_name { #field_name: value.0.to_string() }
                }
            }

            impl From<crate::utils::header_rejection::HeaderRejection> for #name {
                fn from(value: crate::utils::header_rejection::HeaderRejection) -> Self {
                    Self::#variant_name { #field_name: value.0 }
                }
            }
        }
    });

    let from_incorrect_id = incorrect_id_variant.map(|(variant_name, field_name)| {
        quote! {
            impl From<std::num::ParseIntError> for #name {
                fn from(value: std::num::ParseIntError) -> Self {
                    Self::#variant_name { #field_name: value.to_string() }
                }
            }
        }
    });

    quote! {
        impl screw_api::response::ApiResponseContentBase for #name {
            fn status_code(&self) -> hyper::StatusCode {
                match self { #(#status_arms),* }
            }
        }

        impl screw_api::response::ApiResponseContentFailure for #name {
            fn identifier(&self) -> &'static str {
                match self { #(#identifier_arms),* }
            }
            fn reason(&self) -> Option<String> {
                match self { #(#reason_arms),* }
            }
        }

        #from_auth
        #from_database
        #from_incorrect_id
        #from_body
    }
    .into()
}
