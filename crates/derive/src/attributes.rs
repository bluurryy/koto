use syn::{Attribute, LitStr, Path, parse_quote};

pub(crate) struct KotoAttributes {
    pub type_name: Option<String>,
    pub use_copy: bool,
    pub runtime: Path,
    pub memory: Path,
    pub trace_ignore: bool,
}

impl Default for KotoAttributes {
    fn default() -> Self {
        Self {
            type_name: None,
            use_copy: false,
            runtime: parse_quote! { ::koto::runtime },
            memory: parse_quote! { ERROR }, // this will be replaced when parsing
            trace_ignore: false,
        }
    }
}

pub(crate) fn koto_derive_attributes(attrs: &[Attribute]) -> KotoAttributes {
    let mut result = KotoAttributes::default();
    let mut memory = None::<Path>;

    for attr in attrs.iter().filter(|a| a.path().is_ident("koto")) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("type_name") {
                let value = meta.value()?;
                let s: LitStr = value.parse()?;
                result.type_name = Some(s.value());
                Ok(())
            } else if meta.path.is_ident("use_copy") {
                result.use_copy = true;
                Ok(())
            } else if meta.path.is_ident("runtime") {
                result.runtime = meta.value()?.parse()?;
                Ok(())
            } else if meta.path.is_ident("memory") {
                memory = Some(meta.value()?.parse()?);
                Ok(())
            } else if meta.path.is_ident("trace") {
                meta.parse_nested_meta(|meta| {
                    if meta.path.is_ident("ignore") {
                        result.trace_ignore = true;
                        Ok(())
                    } else {
                        Err(meta.error("unsupported option for trace attribute"))
                    }
                })
            } else {
                Err(meta.error("unsupported koto attribute"))
            }
        })
        .expect("failed to parse koto attribute");
    }

    result.memory = memory.unwrap_or_else(|| {
        let runtime = &result.runtime;
        parse_quote!(#runtime::memory)
    });

    result
}
