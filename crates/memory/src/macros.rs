#![allow(unused_macros, unused_imports)]

/// A macro that asserts that exactly one of the features is enabled.
macro_rules! assert_exactly_one_feature_active {
    // Macro input.
    ($($features:literal),*) => {
        $crate::assert_exactly_one_feature_active! { @
            prev = []
            rest = [$($features)*]
            list = [$($features)*]
            comb = []
        }
    };
    // Collecting the combinations in `comb`...
    (@
        prev = [$($prev:literal)*]
        rest = [$feat:literal $($rest:literal)*]
        list = $list:tt
        comb = [$($comb:tt)*]
    ) => {
        $crate::assert_exactly_one_feature_active! { @
            prev = [$($prev)* $feat]
            rest = [$($rest)*]
            list = $list
            comb = [
                $({
                    a = $prev
                    b = $feat
                })*
                $($comb)*
            ]
        }
    };
    // We're done building the combinations, now we emit code.
    (@
        prev = [$($prev:literal)*]
        rest = []
        list = [$($feat:literal)*]
        comb = [$({
            a = $a:literal
            b = $b:literal
        })*]
    ) => {
        // compile_error!(concat!(
        //     "debug combinations: \n",
        //     $(
        //         "a: ", $a, ", ",
        //         "b: ", $b, ", ",
        //         "\n",
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
        #[cfg(any(
            $(all(feature = $a, feature = $b)),*
        ))]
        compile_error!(concat!(
            "The ",
            $crate::assert_exactly_one_feature_active!(@comma_separated_list "and" $($feat)*),
            " features are mutually exclusive and cannot be enabled at the same time!\n\
            You must choose only one of them.",
        ));
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
macro_rules! assert_exactly_one_ptr_feature {
    () => {
        $crate::assert_exactly_one_feature_active!("rc", "gc", "arc", "agc");
    };
}

/// Selects code at compile-time based on the active features.
///
/// For a version that works in expression contexts, see [`feature_match!`].
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
        previous_features = []
        remaining_arms = [{
            features = [$($current_features:literal)*]
            body = [$($current_body:tt)*]
        } $($remaining_arms:tt)*]
        $(fallback = [$($fallback:tt)*])?
    ) => {
        #[cfg(
            // matching one of the current features
            any($(feature = $current_features),*)
        )]
        $crate::feature_select!(@identity $($current_body)*);

        $crate::feature_select!(@iter
            previous_features = [$($current_features)*]
            remaining_arms = [$($remaining_arms)*]
            $(fallback = [$($fallback)*])?
        );
    };
    (@iter
        previous_features = [$($previous_features:literal)+]
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

/// Selects `item`s at compile-time based on the active features.
///
/// Unlike [`feature_select`] this works with `doc(auto_cfg)` which makes the feature gating
/// show up in the documentation.
macro_rules! feature_select_item {
    (
        $($($features:literal)|* => {$body:item})*
        $(_ => {$fallback:item})?
    ) => {
        $crate::feature_select_item!(@iter
            previous_features = []
            remaining_arms = [$({
                features = [$($features)*]
                body = [$body]
            })*]
            $(fallback = [$fallback])?
        );
    };
    (@iter
        previous_features = []
        remaining_arms = [{
            features = [$($current_features:literal)*]
            body = [$current_body:item]
        } $($remaining_arms:tt)*]
        $(fallback = [$fallback:item])?
    ) => {
        #[cfg(
            // matching one of the current features
            any($(feature = $current_features),*)
        )]
        $current_body

        $crate::feature_select_item!(@iter
            previous_features = [$($current_features)*]
            remaining_arms = [$($remaining_arms)*]
            $(fallback = [$($fallback)*])?
        );
    };
    (@iter
        previous_features = [$($previous_features:literal)+]
        remaining_arms = [{
            features = [$($current_features:literal)*]
            body = [$current_body:item]
        } $($remaining_arms:tt)*]
        $(fallback = [$fallback:item])?
    ) => {
        #[cfg(all(
            // not any previous arm
            not(any(
                $(feature = $previous_features),*
            )),
            // and matching one of the current features
            any($(feature = $current_features),*)
        ))]
        $current_body

        $crate::feature_select_item!(@iter
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
        $fallback
    };
    (@iter
        previous_features = [$($previous_features:literal)*]
        remaining_arms = []
    ) => {};
}

/// Selects code at compile-time based on the active features.
///
/// For a version that works in item contexts, see [`feature_select!`].
macro_rules! feature_match {
    (
        $($($features:literal)|* => {$($body:tt)*})*
        _ => { $($fallback:tt)* }
    ) => {
        $crate::feature_match! { @iter
            previous_features = []
            remaining_arms = [$({
                features = [$($features)*]
                body = [$($body)*]
            })*]
            fallback = [$($fallback)*]
            accumulator = []
        }
    };
    (@iter
        previous_features = [$($previous_features:literal)*]
        remaining_arms = [{
            features = [$($current_features:literal)*]
            body = [$($current_body:tt)*]
        } $($remaining_arms:tt)*]
        fallback = [$($fallback:tt)*]
        accumulator = [$($accumulator:tt)*]
    ) => {
        $crate::feature_match! { @iter
            previous_features = [$($previous_features)* $($current_features)*]
            remaining_arms = [$($remaining_arms)*]
            fallback = [$($fallback)*]
            accumulator = [$($accumulator)* {
                not_any = [$($previous_features)*]
                any = [$($current_features)*]
                body = [$($current_body)*]
            }]
        }
    };
    (@iter
        previous_features = [$($previous_features:literal)*]
        remaining_arms = []
        fallback = [$($fallback:tt)*]
        accumulator = [$($accumulator:tt)*]
    ) => {
        $crate::feature_match!{ @emit $($accumulator)* {
            not_any = [$($previous_features)*]
            any = []
            body = [$($fallback)*]
        }}
    };
    (@emit $({
        not_any = [$($($not_any:literal)+)?]
        any = [$($($any:literal)+)?]
        body = [$($body:tt)*]
    })*) => {
        match () {
            $(
                $(#[cfg(not(any($(feature = $not_any),+)))])?
                $(#[cfg(any($(feature = $any),+))])?
                _ => { $($body)* }
            )*
        }
    };
}

/// Like [`feature_match!`], but with an preset fallback arm that panics.
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

/// Emits the `$then` tokens if exactly one of the `$feat`ures is active.
///
/// Otherwise it emits the `$else` tokens.
macro_rules! if_exactly_one_feature_active {
    // Macro input.
    (
        if $($feat:literal)|+ {
            $($then:tt)*
        } else {
            $($else:tt)*
        }
    ) => {
        $crate::if_exactly_one_feature_active! { @
            prev = []
            rest = [$($feat)+]
            list = [$($feat)+]
            then = [$($then)*]
            else = [$($else)*]
            comb = []
        }
    };
    // Collecting the combinations in `comb`...
    (@
        prev = [$($prev:literal)*]
        rest = [$feat:literal $($rest:literal)*]
        list = $list:tt
        then = [$($then:tt)*]
        else = [$($else:tt)*]
        comb = [$($comb:tt)*]
    ) => {
        $crate::if_exactly_one_feature_active! { @
            prev = [$($prev)* $feat]
            rest = [$($rest)*]
            list = $list
            then = [$($then)*]
            else = [$($else)*]
            comb = [$({a = $prev b = $feat})* $($comb)*]
        }
    };
    // We're done building the combinations, now we emit code.
    (@
        prev = [$($prev:literal)*]
        rest = []
        list = [$($feat:literal)*]
        then = [$($then:tt)*]
        else = [$($else:tt)*]
        comb = [$({a = $a:literal b = $b:literal})*]
    ) => {
        // If there are no issues, emit the `$then` tokens.
        #[cfg(all(
            // Some feature must be enabled...
            any($(feature = $feat),*),
            // but only one...
            not(any($(all(feature = $a, feature = $b)),*))
        ))]
        $crate::if_exactly_one_feature_active!(@identity $($then)*);

        // If there are multiple or no features enabled, emit the `$else` tokens.
        #[cfg(any(
            // If multiple features are enabled...
            any($(all(feature = $a, feature = $b)),*),
            // or if none of the features are enabled...
            all($(not(feature = $feat)),*)
        ))]
        $crate::if_exactly_one_feature_active!(@identity $($else)*);
    };
    // Builds a comma separated list of quoted strings like `"a", "b", or "c"`.
    (@comma_separated_list $final_delim:literal $a:literal $b:literal) => {
        concat!("\"", $a, "\", ", $final_delim, " \"", $b, "\"")
    };
    (@comma_separated_list $final_delim:literal $a:literal $b:literal $($rest:literal)+) => {
        concat!("\"", $a, "\", ",
            $crate::if_exactly_one_feature_active!(@comma_separated_list $final_delim $b $($rest)*)
        )
    };
    // Returns the tokens as-is. This is used to `#[cfg(...)]` the expansion of multiple tokens.
    (@identity
        $($tt:tt)*
    ) => {
        $($tt)*
    };
}

if_exactly_one_feature_active! {
    if "rc" | "gc" | "arc" | "agc" {
        /// Like [`feature_select!`], but emits the fallback tokens
        /// when multiple features are active.
        macro_rules! exactly_one_feature_select {
            (
                $($($features:literal)|* => {$($body:tt)*})*
                $(_ => {$($fallback:tt)*})?
            ) => {
                $crate::feature_select! {
                    $($($features)|* => {$($body)*})*
                    $(_ => {$($fallback)*})?
                }
            };
        }

        /// Like [`feature_match!`], but emits the fallback tokens
        /// when multiple features are active.
        macro_rules! exactly_one_feature_match {
            (
                $($($features:literal)|* => {$($body:tt)*})*
                $(_ => {$($fallback:tt)*})?
            ) => {
                $crate::feature_match! {
                    $($($features)|* => {$($body)*})*
                    $(_ => {$($fallback)*})?
                }
            };
        }
    } else {
        macro_rules! exactly_one_feature_select {
            (
                $($($features:literal)|* => {$($body:tt)*})*
                $(_ => {$($fallback:tt)*})?
            ) => {
                $($($fallback)*)?
            };
        }
        macro_rules! exactly_one_feature_match {
            (
                $($($features:literal)|* => {$($body:tt)*})*
                $(_ => {$($fallback:tt)*})?
            ) => {
                $($($fallback)*)?
            };
        }
    }
}

/// Like [`exactly_one_feature_match!`], but with an preset fallback arm that panics.
macro_rules! exactly_one_feature_match_or_panic {
    (
        $($($features:literal)|* => {$($body:tt)*})+
    ) => {
        $crate::exactly_one_feature_match! {
            $($($features)|* => {$($body)*})+
            _ => { unimplemented!() }
        }
    };
}

pub(crate) use assert_exactly_one_feature_active;
pub(crate) use assert_exactly_one_ptr_feature;
pub(crate) use exactly_one_feature_match;
pub(crate) use exactly_one_feature_match_or_panic;
pub(crate) use exactly_one_feature_select;
pub(crate) use feature_match;
pub(crate) use feature_match_or_panic;
pub(crate) use feature_select;
pub(crate) use feature_select_item;
pub(crate) use if_exactly_one_feature_active;
