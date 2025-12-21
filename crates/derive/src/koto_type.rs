use crate::attributes::koto_container_attributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

pub(crate) fn derive_koto_type(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match derive_koto_type_inner(input.into()) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn derive_koto_type_inner(input: TokenStream) -> Result<TokenStream> {
    let input = syn::parse2::<DeriveInput>(input)?;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let attributes = koto_container_attributes(&input.attrs)?;

    let name = input.ident;

    let type_name = attributes
        .type_name
        .unwrap_or_else(|| quote!(#name).to_string());

    let runtime = attributes.runtime;

    let result = quote! {
        #[automatically_derived]
        impl #impl_generics #runtime::KotoType for #name #ty_generics #where_clause {
            fn type_static() -> &'static str {
                #type_name
            }

            fn type_string(&self) -> #runtime::KString {
                #runtime::lazy!(#runtime::KString; #type_name)
            }
        }
    };

    Ok(result)
}
