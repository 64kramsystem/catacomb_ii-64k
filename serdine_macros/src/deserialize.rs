use crate::bail::bail;
use crate::field_data::FieldData;

use proc_macro2::Ident;
use quote::quote;
use syn::{self, parse2, Data, DataStruct, DeriveInput, Fields, Lit, Meta, MetaNameValue};

type TokenStream2 = proc_macro2::TokenStream;

const DESERIALIZE_ATTR: &str = "deserialize";
const SERIALIZE_ATTR: &str = "serialize";

pub(crate) fn impl_deserialize(input: impl Into<TokenStream2>) -> syn::Result<TokenStream2> {
    let ast: DeriveInput = parse2(input.into())?;

    let fields_data = find_fields(&ast)?;

    let deserialize_impl = impl_deserialize_trait(&ast, fields_data)?;

    Ok(quote!(
        #deserialize_impl
    ))
}

fn find_fields(ast: &'_ DeriveInput) -> syn::Result<Vec<FieldData>> {
    if let Data::Struct(DataStruct {
        fields: Fields::Named(fields),
        ..
    }) = &ast.data
    {
        let mut fields_data = vec![];

        for f in &fields.named {
            // Fields are named, so an ident is necessarily found.
            let mut field_data = FieldData::new(f.ident.clone().unwrap());

            for attr in &f.attrs {
                let attr_meta = match attr.parse_meta() {
                    Ok(meta) => meta,
                    Err(error) => bail!(error),
                };

                if let Meta::NameValue(MetaNameValue {
                    ref path, ref lit, ..
                }) = attr_meta
                {
                    if path.is_ident(DESERIALIZE_ATTR) {
                        if let Lit::Str(lit_val) = lit {
                            field_data.deserialization_fn = Some(lit_val.to_owned());
                        } else {
                            bail!("The `deserialize` attribute requires a string literal");
                        }
                    } else if path.is_ident(SERIALIZE_ATTR) {
                        if let Lit::Str(lit_val) = lit {
                            field_data.serialization_fn = Some(lit_val.to_owned());
                        } else {
                            bail!("The `serialize` attribute requires a string literal");
                        }
                    }
                }
            }

            fields_data.push(field_data);
        }

        Ok(fields_data)
    } else {
        bail!("Unexpected input; named fields expected")
    }
}

fn impl_deserialize_trait(
    ast: &'_ DeriveInput,
    fields_data: Vec<FieldData>,
) -> syn::Result<TokenStream2> {
    let type_name = &ast.ident;

    let fields_deserialization = fields_data.iter().map(
        |FieldData {
             field,
             deserialization_fn,
             ..
         }| {
            let quoted_deserialization_fn = if let Some(deserialization_fn) = deserialization_fn {
                let deserialization_fn =
                    Ident::new(&deserialization_fn.value(), deserialization_fn.span());
                quote! { #deserialization_fn(&mut r) }
            } else {
                quote! { serdine::Deserialize::deserialize(&mut r) }
            };

            quote! { let #field = #quoted_deserialization_fn; }
        },
    );

    let self_fields = fields_data
        .iter()
        .map(|FieldData { field, .. }| quote! { #field, });

    Ok(quote!(
        impl serdine::Deserialize for #type_name {
            fn deserialize<R: std::io::Read>(mut r: R) -> Self {
                #(#fields_deserialization)*

                Self {
                    #(#self_fields)*
                }
            }
        }
    ))
}
