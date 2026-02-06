use crate::attributes::{koto_container_attributes, koto_field_attributes};
use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn derive_koto_trace(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    match derive_koto_trace_inner(input) {
        Ok(tokens) => tokens.into(),
        Err(error) => error.into_compile_error().into(),
    }
}

fn derive_koto_trace_inner(input: proc_macro::TokenStream) -> syn::Result<TokenStream> {
    if cfg!(not(any(feature = "gc", feature = "agc"))) {
        // do nothing if no gc feature is enabled
        return Ok(quote!());
    }

    let di = syn::parse::<DeriveInput>(input)?;
    let mut s = synstructure::Structure::try_new(&di)?;

    let container_attrs = koto_container_attributes(&s.ast().attrs)?;
    let memory = &container_attrs.memory;

    let body = if container_attrs.trace_ignore {
        // With `trace(ignore)` no additional bounds are added.
        s.add_bounds(synstructure::AddBounds::None);
        quote!()
    } else {
        // Every field must implement `Trace` (but not necessarily the generics).
        s.add_bounds(synstructure::AddBounds::Fields);

        // There is no `try_filter` so we store the parse error here, to return it
        // after the `filter` call.
        let mut field_attr_parse_error = None;

        // Filter out fields with `#[koto(trace(ignore))]`
        s.filter(|bi| match koto_field_attributes(&bi.ast().attrs) {
            Ok(field_attrs) => !field_attrs.trace_ignore,
            Err(error) => {
                field_attr_parse_error.get_or_insert(error);
                false
            }
        });

        if let Some(error) = field_attr_parse_error {
            return Err(error);
        }

        let body = s.each(|bi| {
            quote! {
                #memory::dumpster::TraceWith::accept(#bi, visitor)?;
            }
        });

        quote!(match *self { #body })
    };

    Ok(s.gen_impl(quote! {
        gen unsafe impl<__V: #memory::dumpster::Visitor> #memory::dumpster::TraceWith<__V> for @Self {
            #[inline]
            fn accept(&self, visitor: &mut __V) -> ::core::result::Result<(), ()> {
                #body
                ::core::result::Result::Ok(())
            }
        }
    }))
}
