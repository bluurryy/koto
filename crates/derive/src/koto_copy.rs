use crate::attributes::koto_container_attributes;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Result};

pub(crate) fn derive_koto_copy(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match derive_koto_copy_inner(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn derive_koto_copy_inner(input: proc_macro::TokenStream) -> Result<TokenStream> {
    let input = syn::parse::<DeriveInput>(input)?;
    let name = input.ident;
    let (impl_generics, ty_generics, generic_where_clause) = input.generics.split_for_impl();

    let attributes = koto_container_attributes(&input.attrs)?;
    let (required_trait, copy_impl) = if attributes.use_copy {
        (quote! {Copy}, quote! {(*self).into()})
    } else {
        (quote! {Clone}, quote! {self.clone().into()})
    };

    let object_where_clause = quote! { #name #ty_generics: KotoObject + #required_trait };
    let where_clause = if let Some(generic_where_clause) = generic_where_clause {
        if generic_where_clause.predicates.trailing_punct() {
            quote! { #generic_where_clause #object_where_clause }
        } else {
            quote! { #generic_where_clause, #object_where_clause }
        }
    } else {
        quote! { where #object_where_clause }
    };

    let result = quote! {
        #[automatically_derived]
        impl #impl_generics KotoCopy for #name #ty_generics
            #where_clause
        {
            fn copy(&self) -> KObject {
                #copy_impl
            }
        }
    };

    Ok(result)
}
