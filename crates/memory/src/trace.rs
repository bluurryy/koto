use crate::macros::{exactly_one_feature_select, feature_select};

feature_select! {
    "gc" | "agc" => {
        macro_rules! dumpster_trace_reference {
            // dumpster is in the dependencies, so we can link to it
            () => { "[`dumpster::Trace`]" };
        }
    }
    _ => {
        macro_rules! dumpster_trace_reference {
            () => { "`dumpster::Trace`" };
        }
    }
}

macro_rules! koto_trace_docs {
    () => {
        concat!(
            "\
A trait that powers garbage collection

It is implemented for all types that satisfy its bounds.
The bounds depend on the active feature:
- **`rc`** or **`arc`** — requires no bounds
- **`gc`** or **`agc`** — requires ", dumpster_trace_reference!(), "

You can implement this trait for your own type using <code>[#\\[derive(KotoTrace)\\]][derive]</code>.

[derive]: macro@crate::KotoTrace
"
        )
    };
}

exactly_one_feature_select! {
    "gc" | "agc" => {
        #[doc = koto_trace_docs!()]
        pub trait KotoTrace: dumpster::Trace {}
        impl<T: ?Sized + dumpster::Trace> KotoTrace for T {}
    }
    _ => {
        #[doc = koto_trace_docs!()]
        pub trait KotoTrace {}
        impl<T: ?Sized> KotoTrace for T {}
    }
}
