mod derive;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};
use darling::FromDeriveInput;

#[proc_macro_derive(Model, attributes(model))]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let meta_model = match derive::model::MetaModel::from_derive_input(&parse_macro_input!(input as DeriveInput)) {
        Ok(m) => m,
        Err(e) => return TokenStream::from(e.write_errors()),
    };
    meta_model.expand()
}