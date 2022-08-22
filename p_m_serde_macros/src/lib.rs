mod bail;
mod deserialize;
mod field_data;

use deserialize::impl_deserialize;
use proc_macro::TokenStream;
use syn::{self};

#[proc_macro_derive(Deserialize, attributes(deserialize))]
pub fn deserialize(input: TokenStream) -> TokenStream {
    let deserialize_impl = impl_deserialize(input);

    deserialize_impl
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
