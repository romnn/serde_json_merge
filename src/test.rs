use crate::iter::{KeyValueIter, Traverser};
use crate::IndexPath;
use serde_json::Value;
use std::fmt;

pub trait ValueExt {
    fn try_keys(&self) -> Option<Vec<String>>;
}

impl<'a> ValueExt for Option<&'a Value> {
    fn try_keys(&self) -> Option<Vec<String>> {
        self.and_then(|value| match value {
            Value::Object(map) => Some(map.keys().cloned().collect::<Vec<String>>()),
            _ => None,
        })
    }
}

pub trait CollectCloned {
    fn collect_cloned(self) -> Vec<(IndexPath, Value)>;
}

impl<'a, Iter, T> CollectCloned for Iter
where
    Iter: IntoIterator<IntoIter = KeyValueIter<'a, T>>,
    T: Traverser + Clone,
{
    fn collect_cloned(self) -> Vec<(IndexPath, Value)> {
        self.into_iter()
            .map(|(index, value)| (index, value.clone()))
            .collect::<Vec<(IndexPath, Value)>>()
    }
}

struct Pattern<'a>(&'a str);

impl fmt::Debug for Pattern<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

pub fn assert_failed(
    left: &dyn fmt::Debug,
    op: &str,
    right: &dyn fmt::Debug,
    args: Option<fmt::Arguments<'_>>,
) -> ! {
    match args {
        Some(args) => panic!(
            r#"assertion failed: `(left {} right)`
  left: `{:?}`,
 right: `{:?}`: {:?}"#,
            op, left, right, args
        ),
        None => panic!(
            r#"assertion failed: `(left {} right)`
  left: `{:?}`,
 right: `{:?}`"#,
            op, left, right
        ),
    }
}

#[allow(dead_code)]
pub fn assert_matches_failed<T: fmt::Debug + ?Sized>(
    left: &T,
    right: &str,
    args: Option<fmt::Arguments<'_>>,
) -> ! {
    assert_failed(&left, "matches", &Pattern(right), args);
}

#[allow(unused_macros)]
macro_rules! assert_matches {
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {}
            ref left_val => {
                $crate::test::assert_matches_failed(
                    left_val,
                    std::stringify!($($pattern)|+ $(if $guard)?),
                    std::option::Option::None
                );
            }
        }
    };
    ($left:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )?, $($arg:tt)+) => {
        match $left {
            $( $pattern )|+ $( if $guard )? => {}
            ref left_val => {
                $crate::test::assert_matches_failed(
                    left_val,
                    std::stringify!($($pattern)|+ $(if $guard)?),
                    std::option::Option::Some(std::format_args!($($arg)+))
                );
            }
        }
    };
}

#[allow(unused_imports)]
pub(crate) use assert_matches;

macro_rules! assert_eq_ordered {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !$crate::sort::PartialEqOrdered::eq(left_val, right_val) {
                    $crate::test::assert_failed(
                        left_val,
                        "equals (ordered)",
                        right_val,
                        std::option::Option::None
                    );
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if !$crate::sort::PartialEqOrdered::eq(left_val, right_val) {
                    $crate::test::assert_failed(
                        left_val,
                        "equals (ordered)",
                        right_val,
                        std::option::Option::Some(std::format_args!($($arg)+))
                    );
                }
            }
        }
    };
}

pub(crate) use assert_eq_ordered;

macro_rules! assert_ne_ordered {
    ($left:expr, $right:expr $(,)?) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if $crate::sort::PartialEqOrdered::eq(left_val, right_val) {
                    $crate::test::assert_failed(
                        left_val,
                        "does not equal (ordered)",
                        right_val,
                        std::option::Option::None
                    );
                }
            }
        }
    };
    ($left:expr, $right:expr, $($arg:tt)+) => {
        match (&$left, &$right) {
            (left_val, right_val) => {
                if $crate::sort::PartialEqOrdered::eq(left_val, right_val) {
                    $crate::test::assert_failed(
                        left_val,
                        "does not equal (ordered)",
                        right_val,
                        std::option::Option::Some(std::format_args!($($arg)+))
                    );
                }
            }
        }
    }
}

pub(crate) use assert_ne_ordered;
