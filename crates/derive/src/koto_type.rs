use crate::{PREFIX_STATIC, attributes::koto_derive_attributes};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{DeriveInput, parse_macro_input};

pub fn derive_koto_type(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let attributes = koto_derive_attributes(&input.attrs);

    let name = input.ident;
    let type_name = attributes
        .type_name
        .unwrap_or_else(|| quote!(#name).to_string());

    let type_string_name = format_ident!("{PREFIX_STATIC}TYPE_STRING_{}", type_name.to_uppercase());

    let result = quote! {
        #[automatically_derived]
        impl #impl_generics KotoType for #name #ty_generics #where_clause {
            fn type_static() -> &'static str {
                #type_name
            }

            fn type_string(&self) -> KString {
                #type_string_name.with(KString::clone)
            }
        }

        #[automatically_derived]
        thread_local! {
            static #type_string_name: KString = #type_name.into();
        }
    };

    result.into()
}
