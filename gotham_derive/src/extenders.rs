use syn;
use quote;

use helpers::ty_params;

pub fn bad_request_static_response_extender(ast: &syn::DeriveInput) -> quote::Tokens {
    let (name, borrowed, where_clause) = ty_params(&ast);

    quote! {
        impl #borrowed gotham::router::response::extender::StaticResponseExtender for #name #borrowed
        #where_clause
        {
            fn extend(_state: &mut gotham::state::State, res: &mut hyper::Response) {
                res.set_status(hyper::StatusCode::BadRequest);
            }
        }
    }
}
