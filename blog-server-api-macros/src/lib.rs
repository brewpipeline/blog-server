use proc_macro::TokenStream;

mod failure;
mod request;
mod success;

#[proc_macro_derive(ApiFailure, attributes(failure))]
pub fn api_failure(input: TokenStream) -> TokenStream {
    failure::derive(input)
}

#[proc_macro_derive(ApiSuccess, attributes(success))]
pub fn api_success(input: TokenStream) -> TokenStream {
    success::derive(input)
}

#[proc_macro_derive(ApiRequest, attributes(request))]
pub fn api_request(input: TokenStream) -> TokenStream {
    request::derive(input)
}
