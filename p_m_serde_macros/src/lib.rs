use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{self, parse2, Data, DataStruct, DeriveInput, Fields};

type TokenStream2 = proc_macro2::TokenStream;

macro_rules! bail {
    ( $msg:expr $(,)? ) => {
        return syn::Result::<_>::Err(syn::Error::new(Span::call_site(), &$msg))
    };

    ( $msg:expr => $spanned:expr $(,)? ) => {
        return syn::Result::<_>::Err(syn::Error::new_spanned(&$spanned, &$msg))
    };
}

#[proc_macro_derive(Deserialize)]
pub fn deserialize(input: TokenStream) -> TokenStream {
    let deserialize_impl = impl_deserialize(input);

    deserialize_impl
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}

fn impl_deserialize(input: impl Into<TokenStream2>) -> syn::Result<TokenStream2> {
    let ast: DeriveInput = parse2(input.into())?;

    let fields = find_fields(&ast)?;

    let deserialize_impl = impl_deserialize_trait(&ast, fields)?;

    Ok(quote!(
        #deserialize_impl
    ))
}

fn find_fields(ast: &'_ DeriveInput) -> syn::Result<Vec<Ident>> {
    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &ast.data
    {
        let field_names = fields
            .named
            .iter()
            .map(|f| f.ident.clone().unwrap())
            .collect::<Vec<_>>();
        Ok(field_names)
    } else {
        bail!("Unexpected input; named fields expected")
    }
}

fn impl_deserialize_trait(ast: &'_ DeriveInput, fields: Vec<Ident>) -> syn::Result<TokenStream2> {
    let type_name = &ast.ident;

    let field_deserialization = fields
        .iter()
        .map(|field| quote! { let #field = p_m_serde::Deserialize::deserialize(&mut r); });

    let self_fields = fields.iter().map(|field| quote! { #field, });

    Ok(quote!(
        impl p_m_serde::Deserialize for #type_name {
            fn deserialize<R: std::io::Read>(mut r: R) -> Self {
                #(#field_deserialization)*

                Self {
                    #(#self_fields)*
                }
            }
        }
    ))
}
