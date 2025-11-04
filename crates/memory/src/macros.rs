/// A macro that asserts that exactly one of the features is enabled.
macro_rules! assert_mutually_exclusive_features {
    (
        $($feat:literal),+
    ) => {
        assert_mutually_exclusive_features! { @
            prev = []
            rest = [$($feat)+]
        }
    };
    (@
        prev = [$($prev:literal)*]
        rest = [$feat:literal $($rest_feat:literal)*]
    ) => {
        assert_mutually_exclusive_features! { @
            prev = [$($prev)* $feat]
            rest = [$($rest_feat)*]
        }
    };
    (@
        prev = [$($prev:literal)*]
        rest = []
    ) => {
        #[cfg(all(
            $(not(feature = $prev)),*
        ))]
        compile_error!(concat!(
            "You must enable exactly one of the ",
            assert_mutually_exclusive_features!(@comma_separated_list "or" $($prev)*),
            " features!"
        ));

        assert_mutually_exclusive_features! { @dupes $($prev)* }
    };
    (@identity
        $($tt:tt)*
    ) => {
        $($tt)*
    };
    (@dupes $($feat:literal)*) => {
        assert_mutually_exclusive_features!(@dupes
            prev = []
            rest = [$($feat)*]
            list = [$($feat)*]
        );
    };
    (@dupes
        prev = [$($prev:literal)*]
        rest = []
        list = $list:tt
    ) => {    };
    (@dupes
        prev = [$($prev:literal)*]
        rest = [$feat:literal $($rest:literal)*]
        list = $list:tt
    ) => {
        $(
            assert_mutually_exclusive_features!(@dupes_check
                a = $prev
                b = $feat
                list = $list
            );
        )*

        assert_mutually_exclusive_features!(@dupes
            prev = [$($prev)* $feat]
            rest = [$($rest)*]
            list = $list
        );
    };
    (@dupes_check
        a = $a:literal
        b = $b:literal
        list = [$($list:literal)*]
    ) => {
        #[cfg(all(feature = $a, feature = $b))]
        compile_error!(concat!("The \"", $a, "\" and \"", $b, "\" features are mutually exclusive and cannot be enabled at the same time!\n\
            You must choose one of ",
            assert_mutually_exclusive_features!(@comma_separated_list "or" $($list)*), "."
        ));
    };
    (@comma_separated_list $final_delim:literal $a:literal $b:literal) => {
        concat!("\"", $a, "\", ", $final_delim, " \"", $b, "\"")
    };
    (@comma_separated_list $final_delim:literal $a:literal $b:literal $($rest:literal)+) => {
        concat!("\"", $a, "\", ",
            assert_mutually_exclusive_features!(@comma_separated_list $final_delim $b $($rest)*)
        )
    };
}

/// A macro that allows matching over exclusively enabled features.
///
/// Note that simply feature gating like this:
/// ```ignore
/// #[cfg(feature = "rc")]
/// pub use crate::rc::*;
/// ```
/// results in poor error messages because errors like
/// ```text
/// error[E0428]: the name `make_ptr` is defined multiple times
/// ```
/// show up before our custom compile error message.
///
/// This macro instead `cfg`s out the branches on error to reduce
/// the amount of uninteresting error messages.
/// ```
macro_rules! exclusive_feature_select {
    (
        $($($features:literal)|* => {$($body:tt)*})+
        $(_ => {$($fallback:tt)*})?
    ) => {
        $crate::exclusive_feature_select! { @iter
            remaining_arms = [
                $({
                    features = [$($features)*]
                    body = [$($body)*]
                })+
            ]
            remaining_features = []
            current_body = []
            accumulator = []
            fallback = [$($($fallback)*)?]
        }
    };
    (@iter
        remaining_arms = [
            $({
                features = [$($next_arm_feature:literal)*]
                body = [$($next_arm_body:tt)*]
            })*
        ]
        remaining_features = [
            $feature:literal $($next_feature:literal)*
        ]
        current_body = [$($body:tt)*]
        accumulator = [$({
            yes = $accu_yes:literal
            no = [$($accu_no:literal)*]
            body = [$($accu_body:tt)*]
        })*]
        fallback = [$($fallback:tt)*]
    ) => {
        $crate::exclusive_feature_select! {
            @iter
            remaining_arms = [
                $({
                    features = [$($next_arm_feature)*]
                    body = [$($next_arm_body)*]
                })*
            ]
            remaining_features = [
                $($next_feature)*
            ]
            current_body = [$($body)*]
            accumulator = [$({
                yes = $accu_yes
                no = [$($accu_no)*]
                body = [$($accu_body)*]
            })* {
                yes = $feature
                no = [
                    $($accu_yes)*
                    $($next_feature)*
                    $($($next_arm_feature)*)*
                ]
                body = [$($body)*]
            }]
            fallback = [$($fallback)*]
        }
    };
    (@iter
        remaining_arms = [
            {
                features = [$($arm_feature:literal)*]
                body = [$($arm_body:tt)*]
            }
            $({
                features = [$($next_arm_feature:literal)*]
                body = [$($next_arm_body:tt)*]
            })*
        ]
        remaining_features = [

        ]
        current_body = [$($body:tt)*]
        accumulator = [$({
            yes = $accu_yes:literal
            no = [$($accu_no:literal)*]
            body = [$($accu_body:tt)*]
        })*]
        fallback = [$($fallback:tt)*]
    ) => {
        $crate::exclusive_feature_select! { @iter
            remaining_arms = [
                $({
                    features = [$($next_arm_feature)*]
                    body = [$($next_arm_body)*]
                })*
            ]
            remaining_features = [
                $($arm_feature)*
            ]
            current_body = [$($arm_body)*]
            accumulator = [$({
                yes = $accu_yes
                no = [$($accu_no)*]
                body = [$($accu_body)*]
            })*]
            fallback = [$($fallback)*]
        }
    };
    (@iter
        remaining_arms = [

        ]
        remaining_features = [

        ]
        current_body = [$($body:tt)*]
        accumulator = [$({
            yes = $accu_yes:literal
            no = [$($accu_no:literal)*]
            body = [$($accu_body:tt)*]
        })*]
        fallback = [$($fallback:tt)*]
    ) => {
        $(
            #[cfg(all(
                feature = $accu_yes
                $(, not(feature = $accu_no))*
            ))]
            $crate::exclusive_feature_select! { @identity $($accu_body)* }
        )*

        #[cfg(not(any(
            $(all(
                feature = $accu_yes
                $(, not(feature = $accu_no))*
            )),*
        )))]
        $crate::exclusive_feature_select! { @identity
            $($fallback)*
        }
    };
    (@identity
        $($tt:tt)*
    ) => {
        $($tt)*
    };
}

/// Like [`exclusive_feature_select_expr!`] but usable in an expression context.
///
/// If not fallback arm is given, or if it is empty then it will fall back to `unimplemented!()`.
//
// The only difference is that the `cfg`'s are attached to a block `{ ... }` instead of to an
// identity macro invocation `exclusive_feature_select!(@identity ...)`.
macro_rules! exclusive_feature_select_expr {
    (
        $($($features:literal)|* => {$($body:tt)*})+
        $(_ => {$($fallback:tt)*})?
    ) => {{
        $crate::exclusive_feature_select_expr! { @iter
            remaining_arms = [
                $({
                    features = [$($features)*]
                    body = [$($body)*]
                })+
            ]
            remaining_features = []
            current_body = []
            accumulator = []
            fallback = [
                $crate::exclusive_feature_select_expr!{ @or
                    { $($($fallback)*)? }
                    { unimplemented!() }
                }
            ]
        }
    }};
    (@iter
        remaining_arms = [
            $({
                features = [$($next_arm_feature:literal)*]
                body = [$($next_arm_body:tt)*]
            })*
        ]
        remaining_features = [
            $feature:literal $($next_feature:literal)*
        ]
        current_body = [$($body:tt)*]
        accumulator = [$({
            yes = $accu_yes:literal
            no = [$($accu_no:literal)*]
            body = [$($accu_body:tt)*]
        })*]
        fallback = [$($fallback:tt)*]
    ) => {
        $crate::exclusive_feature_select_expr! {
            @iter
            remaining_arms = [
                $({
                    features = [$($next_arm_feature)*]
                    body = [$($next_arm_body)*]
                })*
            ]
            remaining_features = [
                $($next_feature)*
            ]
            current_body = [$($body)*]
            accumulator = [$({
                yes = $accu_yes
                no = [$($accu_no)*]
                body = [$($accu_body)*]
            })* {
                yes = $feature
                no = [
                    $($accu_yes)*
                    $($next_feature)*
                    $($($next_arm_feature)*)*
                ]
                body = [$($body)*]
            }]
            fallback = [$($fallback)*]
        }
    };
    (@iter
        remaining_arms = [
            {
                features = [$($arm_feature:literal)*]
                body = [$($arm_body:tt)*]
            }
            $({
                features = [$($next_arm_feature:literal)*]
                body = [$($next_arm_body:tt)*]
            })*
        ]
        remaining_features = [

        ]
        current_body = [$($body:tt)*]
        accumulator = [$({
            yes = $accu_yes:literal
            no = [$($accu_no:literal)*]
            body = [$($accu_body:tt)*]
        })*]
        fallback = [$($fallback:tt)*]
    ) => {
        $crate::exclusive_feature_select_expr! { @iter
            remaining_arms = [
                $({
                    features = [$($next_arm_feature)*]
                    body = [$($next_arm_body)*]
                })*
            ]
            remaining_features = [
                $($arm_feature)*
            ]
            current_body = [$($arm_body)*]
            accumulator = [$({
                yes = $accu_yes
                no = [$($accu_no)*]
                body = [$($accu_body)*]
            })*]
            fallback = [$($fallback)*]
        }
    };
    (@iter
        remaining_arms = [

        ]
        remaining_features = [

        ]
        current_body = [$($body:tt)*]
        accumulator = [$({
            yes = $accu_yes:literal
            no = [$($accu_no:literal)*]
            body = [$($accu_body:tt)*]
        })*]
        fallback = [$($fallback:tt)*]
    ) => {
        $(
            #[cfg(all(
                feature = $accu_yes
                $(, not(feature = $accu_no))*
            ))]
            { $($accu_body)* }
        )*

        #[cfg(not(any(
            $(all(
                feature = $accu_yes
                $(, not(feature = $accu_no))*
            )),*
        )))]
        { $($fallback)* }
    };
    (@or { } { $($rhs:tt)* }) => {
        $($rhs)*
    };
    (@or { $($lhs:tt)* } { $($rhs:tt)* }) => {
        $($lhs)*
    };
}

pub(crate) use assert_mutually_exclusive_features;
pub(crate) use exclusive_feature_select;
pub(crate) use exclusive_feature_select_expr;
