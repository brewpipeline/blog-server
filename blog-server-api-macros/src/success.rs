use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr, Path, parse_macro_input};

pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let Data::Struct(data) = input.data else {
        return syn::Error::new_spanned(name, "ApiSuccess only applies to structs")
            .to_compile_error()
            .into();
    };

    let mut identifier = None;
    let mut description = None;
    let mut convert_from: Option<Path> = None;

    for attr in &input.attrs {
        if !attr.path().is_ident("success") {
            continue;
        }
        let parsed = attr.parse_nested_meta(|meta| {
            let key = meta
                .path
                .get_ident()
                .ok_or_else(|| meta.error("expected an identifier"))?
                .to_string();
            match key.as_str() {
                "found" => identifier = Some("FOUND"),
                "created" => identifier = Some("CREATED"),
                "ok" => identifier = Some("OK"),
                "description" => {
                    description = Some(meta.value()?.parse::<LitStr>()?.value());
                }
                "from" => convert_from = Some(meta.value()?.parse()?),
                other => return Err(meta.error(format!("unknown success option `{other}`"))),
            }
            Ok(())
        });
        if let Err(error) = parsed {
            return error.to_compile_error().into();
        }
    }

    let Some(identifier) = identifier else {
        return syn::Error::new_spanned(
            name,
            "expected one of #[success(found)], #[success(created)] or #[success(ok)]",
        )
        .to_compile_error()
        .into();
    };
    let Some(description) = description else {
        return syn::Error::new_spanned(name, "expected #[success(description = \"...\")]")
            .to_compile_error()
            .into();
    };

    let (data_type, data_body, from_impls) = match &data.fields {
        Fields::Unit => {
            if let Some(path) = convert_from {
                return syn::Error::new_spanned(
                    path,
                    "a unit success carries no data, so there is nothing to convert into",
                )
                .to_compile_error()
                .into();
            }
            (quote! { () }, quote! { &() }, quote! {})
        }
        Fields::Named(named) if named.named.len() == 1 => {
            let field = named.named.first().unwrap();
            let field_name = &field.ident;
            let field_type = &field.ty;
            let converted = convert_from.map(|path| {
                quote! {
                    impl From<#path> for #name {
                        fn from(value: #path) -> Self {
                            Self { #field_name: value.into() }
                        }
                    }
                }
            });
            (
                quote! { #field_type },
                quote! { &self.#field_name },
                quote! {
                    impl From<#field_type> for #name {
                        fn from(#field_name: #field_type) -> Self {
                            Self { #field_name }
                        }
                    }
                    #converted
                },
            )
        }
        _ => {
            return syn::Error::new_spanned(
                name,
                "expected a unit struct or a struct with exactly one field holding the data",
            )
            .to_compile_error()
            .into();
        }
    };

    quote! {
        impl screw_api::response::ApiResponseContentBase for #name {
            fn status_code(&self) -> hyper::StatusCode {
                hyper::StatusCode::OK
            }
        }

        impl screw_api::response::ApiResponseContentSuccess for #name {
            type Data = #data_type;
            fn identifier(&self) -> &'static str {
                #identifier
            }
            fn description(&self) -> Option<String> {
                Some(#description.to_string())
            }
            fn data(&self) -> &Self::Data {
                #data_body
            }
        }

        #from_impls
    }
    .into()
}
