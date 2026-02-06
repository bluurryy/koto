use syn::{Attribute, LitStr, Path, Result, parse_quote};

pub(crate) struct KotoContainerAttributes {
    pub type_name: Option<String>,
    pub use_copy: bool,
    /// Path to `koto_runtime`
    pub runtime: Path,
    /// Path to `koto_memory`
    pub memory: Path,
    /// Ignore all fields when deriving `KotoTrace`
    pub trace_ignore: bool,
}

impl Default for KotoContainerAttributes {
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

pub(crate) fn koto_container_attributes(attrs: &[Attribute]) -> Result<KotoContainerAttributes> {
    let mut result = KotoContainerAttributes::default();
    let mut memory = None::<Path>;

    for attr in attrs {
        if !attr.path().is_ident("koto") {
            continue;
        }

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
        })?;
    }

    result.memory = memory.unwrap_or_else(|| {
        let runtime = &result.runtime;
        parse_quote!(#runtime::memory)
    });

    Ok(result)
}

#[derive(Default)]
pub(crate) struct KotoFieldAttributes {
    /// Ignore this field when deriving `KotoTrace`
    pub trace_ignore: bool,
}

pub(crate) fn koto_field_attributes(attrs: &[Attribute]) -> Result<KotoFieldAttributes> {
    let mut result = KotoFieldAttributes::default();

    for attr in attrs {
        if !attr.path().is_ident("koto") {
            continue;
        }

        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("trace") {
                meta.parse_nested_meta(|meta| {
                    if meta.path.is_ident("ignore") {
                        result.trace_ignore = true;
                        Ok(())
                    } else {
                        Err(meta.error("unsupported trace attribute"))
                    }
                })
            } else {
                Err(meta.error("unsupported attribute"))
            }
        })?;
    }

    Ok(result)
}
