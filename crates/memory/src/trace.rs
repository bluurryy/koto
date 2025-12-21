use crate::macros::feature_select;

macro_rules! koto_trace_docs {
    ($dumpster_trace_reference:literal) => {
        concat!(
            "\
A trait that powers garbage collection

It is implemented for all types that satisfy its bounds.
The bounds depend on the active feature:
- **`rc`** or **`arc`** — *has no additional bounds*
- **`gc`** or **`agc`** — <code>",
            $dumpster_trace_reference,
            " \
+ 'static</code>

You can implement this trait for your own type using <code>[#\\[derive(KotoTrace)\\]][derive]</code>.

[derive]: macro@crate::KotoTrace
            "
        )
    };
}

feature_select! {
    "rc" | "arc" => {
        #[doc = koto_trace_docs!("dumpster::Trace")]
        pub trait KotoTrace {}
        impl<T: ?Sized> KotoTrace for T {}
    }
    "gc" | "agc" => {
        #[doc = koto_trace_docs!("[dumpster::Trace]")]
        pub trait KotoTrace: dumpster::Trace + 'static {}
        impl<T: ?Sized + dumpster::Trace + 'static> KotoTrace for T {}
    }
}
