/// A macro that asserts that exactly one of the features is enabled.
///
/// Additionally, if the assertion passes, the tokens in the if block are emitted.
/// This prevents additional compile errors being emitted from the inner code.
macro_rules! assert_exactly_one_feature_active {
    // Macro input.
    (
        if $($feat:literal)|+ {
            $($then:tt)*
        }
    ) => {
        $crate::assert_exactly_one_feature_active! { @
            prev = []
            rest = [$($feat)+]
            list = [$($feat)+]
            then = [$($then)*]
            perm = []
        }
    };
    // Collecting the permutations in `perm`...
    (@
        prev = [$($prev:literal)*]
        rest = [$feat:literal $($rest:literal)*]
        list = $list:tt
        then = [$($then:tt)*]
        perm = [$($perm:tt)*]
    ) => {
        $crate::assert_exactly_one_feature_active! { @
            prev = [$($prev)* $feat]
            rest = [$($rest)*]
            list = $list
            then = [$($then)*]
            perm = [
                $({
                    a = $prev
                    b = $feat
                    // The list is copied here into the permutation item
                    // so it can be expanded along with `a` and `b`.
                    list = $list
                })*
                $($perm)*
            ]
        }
    };
    // We're done building the permutations, now we emit code.
    (@
        prev = [$($prev:literal)*]
        rest = []
        list = [$($feat:literal)*]
        then = [$($then:tt)*]
        perm = [$({
            a = $a:literal
            b = $b:literal
            list = [$($list:tt)*]
        })*]
    ) => {
        // compile_error!(concat!(
        //     "debug permutations: \n",
        //     $(
        //         "a: ", $a, ", ",
        //         "b: ", $b, ", ",
        //         "list: ", stringify!($($list)*), "\n",
        //     )*
        // ));

        // If there are none of the features enabled, raise a compile error.
        #[cfg(all($(not(feature = $feat)),*))]
        compile_error!(concat!(
            "You must enable exactly one of the ",
            $crate::assert_exactly_one_feature_active!(@comma_separated_list "or" $($feat)*),
            " features!"
        ));

        // If there are multiple features enabled, raise a compile error.
        $(
            #[cfg(all(feature = $a, feature = $b))]
            compile_error!(concat!("The \"", $a, "\" and \"", $b, "\" features are mutually exclusive and cannot be enabled at the same time!\n\
                You must choose one of ",
                $crate::assert_exactly_one_feature_active!(@comma_separated_list "or" $($list)*),
                "."
            ));
        )*

        // If there are no issues, emit the `$then` tokens.
        #[cfg(all(
            // Some feature must be enabled...
            any($(feature = $feat),*),
            // but only one...
            not(any($(all(feature = $a, feature = $b)),*))
        ))]
        $crate::assert_exactly_one_feature_active!(@identity $($then)*);
    };
    // Builds a comma separated list of quoted strings like `"a", "b", or "c"`.
    (@comma_separated_list $final_delim:literal $a:literal $b:literal) => {
        concat!("\"", $a, "\", ", $final_delim, " \"", $b, "\"")
    };
    (@comma_separated_list $final_delim:literal $a:literal $b:literal $($rest:literal)+) => {
        concat!("\"", $a, "\", ",
            $crate::assert_exactly_one_feature_active!(@comma_separated_list $final_delim $b $($rest)*)
        )
    };
    // Returns the tokens as-is. This is used to `#[cfg(...)]` the expansion of multiple tokens.
    (@identity
        $($tt:tt)*
    ) => {
        $($tt)*
    };
}

/// Asserts that exactly one of the pointer features is enabled.
///
/// Only if the assertion passes will the inner tokens be emitted.
/// This prevents additional compile errors from the inner tokens.
macro_rules! assert_valid_features {
    (
        $($then:tt)*
    ) => {
        $crate::assert_exactly_one_feature_active! {
            if "rc" | "gc" | "arc" | "agc" {
                $($then)*
            }
        }
    };
}

/// Selects code at compile-time based on the active features.
///
/// For a version that works in expression contexts, see [`feature_match!`].
#[allow(unused_macros)]
macro_rules! feature_select {
    (
        $($($features:literal)|* => {$($body:tt)*})*
        $(_ => {$($fallback:tt)*})?
    ) => {
        $crate::feature_select!(@iter
            previous_features = []
            remaining_arms = [$({
                features = [$($features)*]
                body = [$($body)*]
            })*]
            $(fallback = [$($fallback)*])?
        );
    };
    (@iter
        previous_features = [$($previous_features:literal)*]
        remaining_arms = [{
            features = [$($current_features:literal)*]
            body = [$($current_body:tt)*]
        } $($remaining_arms:tt)*]
        $(fallback = [$($fallback:tt)*])?
    ) => {
        #[cfg(all(
            // not any previous arm
            not(any(
                $(feature = $previous_features),*
            )),
            // and matching one of the current features
            any($(feature = $current_features),*)
        ))]
        $crate::feature_select!(@identity $($current_body)*);

        $crate::feature_select!(@iter
            previous_features = [$($previous_features)* $($current_features)*]
            remaining_arms = [$($remaining_arms)*]
            $(fallback = [$($fallback)*])?
        );
    };
    (@iter
        previous_features = [$($previous_features:literal)*]
        remaining_arms = []
        fallback = [$($fallback:tt)*]
    ) => {
        #[cfg(
            // not any previous arm
            not(any($(feature = $previous_features),*))
        )]
        $crate::feature_select!(@identity $($fallback)*);
    };
    (@iter
        previous_features = [$($previous_features:literal)*]
        remaining_arms = []
    ) => {};
    // Returns the tokens as-is. This is used to `#[cfg(...)]` the expansion of multiple tokens.
    (@identity
        $($tt:tt)*
    ) => {
        $($tt)*
    };
}

/// Selects code at compile-time based on the active features.
///
/// For a version that works in item contexts, see [`feature_select!`].
#[allow(unused_macros)]
macro_rules! feature_match {
    (
        $($($features:literal)|* => {$($body:tt)*})*
        $(_ => {$($fallback:tt)*})?
    ) => {{
        $crate::feature_match! { @iter
            previous_features = []
            remaining_arms = [$({
                features = [$($features)*]
                body = [$($body)*]
            })*]
            $(fallback = [$($fallback)*])?
        }
    }};
    (@iter
        previous_features = [$($previous_features:literal)*]
        remaining_arms = [{
            features = [$($current_features:literal)*]
            body = [$($current_body:tt)*]
        } $($remaining_arms:tt)*]
        $(fallback = [$($fallback:tt)*])?
    ) => {
        #[cfg(all(
            // not any previous arm
            not(any(
                $(feature = $previous_features),*
            )),
            // and matching one of the current features
            any($(feature = $current_features),*)
        ))]
        { $($current_body)* }

        $crate::feature_match! { @iter
            previous_features = [$($previous_features)* $($current_features)*]
            remaining_arms = [$($remaining_arms)*]
            $(fallback = [$($fallback)*])?
        }
    };
    (@iter
        previous_features = [$($previous_features:literal)*]
        remaining_arms = []
        fallback = [$($fallback:tt)*]
    ) => {
        #[cfg(
            // not any previous arm
            not(any($(feature = $previous_features),*))
        )]
        { $($fallback)* }
    };
    (@iter
        previous_features = [$($previous_features:literal)*]
        remaining_arms = []
    ) => {};
}

/// Like [`feature_match!`], but with an preset fallback arm that panics.
#[allow(unused_macros)]
macro_rules! feature_match_or_panic {
    (
        $($($features:literal)|* => {$($body:tt)*})+
    ) => {
        $crate::feature_match! {
            $($($features)|* => {$($body)*})+
            _ => { unimplemented!() }
        }
    };
}

pub(crate) use assert_exactly_one_feature_active;
pub(crate) use assert_valid_features;
#[allow(unused_imports)]
pub(crate) use feature_match;
#[allow(unused_imports)]
pub(crate) use feature_match_or_panic;
#[allow(unused_imports)]
pub(crate) use feature_select;
