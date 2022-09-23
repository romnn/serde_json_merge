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

struct Pattern<'a>(&'a str);

impl fmt::Debug for Pattern<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.0)
    }
}

pub fn assert_matches_failed<T: fmt::Debug + ?Sized>(
    left: &T,
    right: &str,
    args: Option<fmt::Arguments<'_>>,
) -> ! {
    assert_failed_inner(&left, &Pattern(right), args);
}

fn assert_failed_inner(
    left: &dyn fmt::Debug,
    right: &dyn fmt::Debug,
    args: Option<fmt::Arguments<'_>>,
) -> ! {
    match args {
        Some(args) => panic!(
            r#"assertion failed: `(left matches right)`
  left: `{:?}`,
 right: `{:?}`: {:?}"#,
            left, right, args
        ),
        None => panic!(
            r#"assertion failed: `(left matches right)`
  left: `{:?}`,
 right: `{:?}`: {:?}"#,
            left, right, args
        ),
    }
}

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

pub(crate) use assert_matches;

// macro_rules! factory_fn {
//     ($name:tt, $t:ty, $val:tt) => {
//         paste::item! {
//             pub fn $name<'a>(value: $t) -> Option<&'a serde_json::Value> {
//                 Some(&serde_json::Value::$val(value.into()))
//             }

//             pub fn [< $name _ mut >]<'a>(value: $t) -> Option<&'a mut serde_json::Value> {
//                 Some(&mut serde_json::Value::$val(value.into()))
//             }
//         }
//     };
// }

// pub fn null<'a>() -> Option<&'a serde_json::Value> {
//     Some(&serde_json::Value::Null)
// }

// pub fn null_mut<'a>() -> Option<&'a mut serde_json::Value> {
//     Some(&mut serde_json::Value::Null)
// }

// factory_fn!(string, &str, String);
// factory_fn!(boolean, bool, Bool);
// factory_fn!(array, Vec<serde_json::Value>, Array);

// macro_rules! string {
//     ( $value:expr ) => {{
//         Some(&serde_json::Value::String($value.into()))
//     }};
// }
// pub(crate) use string;

// macro_rules! string_mut {
//     ( $value:expr ) => {{
//         Some(&mut serde_json::Value::String($value.into()))
//     }};
// }
// pub(crate) use string_mut;

// macro_rules! boolean {
//     ( $value:expr ) => {{
//         Some(&serde_json::Value::Bool($value.into()))
//     }};
// }
// pub(crate) use boolean;

// macro_rules! boolean_mut {
//     ( $value:expr ) => {{
//         Some(&mut serde_json::Value::Bool($value.into()))
//     }};
// }
// pub(crate) use boolean_mut;

// macro_rules! boolean {
//     ( $value:expr ) => {{
//         Some(&serde_json::Value::Bool($value.into()))
//     }};
// }
// pub(crate) use boolean;

// macro_rules! boolean_mut {
//     ( $value:expr ) => {{
//         Some(&mut serde_json::Value::Bool($value.into()))
//     }};
// }
// pub(crate) use boolean_mut;
// }
