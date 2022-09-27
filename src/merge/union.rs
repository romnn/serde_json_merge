use super::Merge;
use crate::index::Path as IndexPath;
use crate::iter::Traverser;
use serde_json::{Map, Value};
use std::borrow::Borrow;

pub trait Union: Sized {
    #[must_use]
    fn union_all<T, V>(iter: impl IntoIterator<Item = V>) -> Self
    where
        T: Traverser,
        V: Borrow<Value>;

    #[must_use]
    fn union_all_by<T, V, F>(iter: impl IntoIterator<Item = V>, union: &mut F) -> Self
    where
        T: Traverser,
        V: Borrow<Value>,
        F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool;

    fn union<T>(&mut self, other: &Self)
    where
        T: Traverser;

    fn union_recursive<T>(&mut self, other: &Self)
    where
        T: Traverser;

    #[inline]
    #[must_use]
    fn into_union<T>(mut self, other: &Self) -> Self
    where
        T: Traverser,
    {
        self.union::<T>(other);
        self
    }

    #[inline]
    #[must_use]
    fn into_union_recursive<T>(mut self, other: &Self) -> Self
    where
        T: Traverser,
    {
        self.union_recursive::<T>(other);
        self
    }
}

impl Union for Value {
    #[inline]
    #[must_use]
    fn union_all<T, V>(values: impl IntoIterator<Item = V>) -> Self
    where
        T: Traverser,
        V: Borrow<Value>,
    {
        Self::union_all_by::<T, V, _>(values, &mut union_func)
    }

    #[inline]
    #[must_use]
    fn union_all_by<T, V, F>(values: impl IntoIterator<Item = V>, union: &mut F) -> Self
    where
        T: Traverser,
        V: Borrow<Value>,
        F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool,
    {
        let mut result = Value::Object(Map::default());
        for v in values {
            result.merge_by_recursive::<T, F>(v.borrow(), union);
        }
        result
    }

    #[inline]
    fn union<T>(&mut self, other: &Self)
    where
        T: Traverser,
    {
        self.merge_by::<T, _>(other, &mut union_func);
    }

    #[inline]
    fn union_recursive<T>(&mut self, other: &Self)
    where
        T: Traverser,
    {
        self.merge_by_recursive::<T, _>(other, &mut union_func);
    }
}

fn union_func(_idx: &IndexPath, this: &mut Value, other: Option<&Value>) -> bool {
    match (this, other) {
        // add new fields when merging two objects
        (&mut Value::Object(ref mut res), Some(&Value::Object(ref other))) => {
            for k in other.keys() {
                res.entry(k.clone()).or_insert(Value::Null);
            }
            true
        }
        // extend array with other array
        (&mut Value::Array(ref mut this), Some(&Value::Array(ref other))) => {
            this.extend(other.clone());
            false
        }
        // never merge in NULL
        (_, Some(&Value::Null)) => false,
        // always merge NULL values
        (this @ &mut Value::Null, Some(other)) => {
            *this = other.clone();
            false
        }
        _ => false,
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::iter::dfs::Dfs;
    use crate::sort::Keys;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    #[test]
    fn union_all_custom_func() {
        let items = [
            json!({
                "title": "This is a title",
                "person" : {
                    "firstName" : "John",
                    "lastName" : "Doe"
                },
                "cities": [ "london", "paris" ]
            }),
            json!({
                "person" : {
                    "firstName" : "John",
                },
            }),
            json!({
                "another" : 1,
            }),
            json!({
                "another" : {
                    "another" : 2,
                },
            }),
            json!({
                "person" : {
                    "firstName" : {
                        "test": "John",
                    },
                    "gender" : {
                        "important": false,
                    },
                },
            }),
        ];

        let mut expected = json!({
            "title": "This is a title",
            "person" : {
                "lastName" : "Doe",
                "firstName" : {
                    "test": "John",
                },
                "gender" : {
                    "important": false,
                },
            },
            "cities": [ "london", "paris" ],
            "another" : {
                "another" : 2,
            },
        });

        expected.sort_keys_recursive::<Dfs>();

        let mut custom_union_func =
            |_idx: &IndexPath, this: &mut Value, other: Option<&Value>| -> bool {
                #[allow(clippy::match_same_arms)]
                match (this, other) {
                    // add new fields when merging two objects
                    (&mut Value::Object(ref mut this), Some(&Value::Object(ref other))) => {
                        for k in other.keys() {
                            this.entry(k.clone()).or_insert(Value::Null);
                        }
                        true
                    }
                    // never overwrite objects
                    (Value::Object(_), Some(_)) => false,
                    // always overwrite non-objects with objects
                    (this, Some(other @ &Value::Object(_))) => {
                        *this = other.clone();
                        false
                    }
                    // never overwrite with NULL
                    (_, Some(&Value::Null)) => false,
                    // always overwrite NULL
                    (this @ &mut Value::Null, Some(other)) => {
                        *this = other.clone();
                        false
                    }
                    // this and other are not objects and other is not NULL
                    (this, Some(other)) => {
                        *this = other.clone();
                        false
                    }
                    _ => false,
                }
            };

        let len = items.len();
        for perm in items.into_iter().permutations(len) {
            let mut union: Value = Union::union_all_by::<Dfs, _, _>(&perm, &mut custom_union_func);
            union.sort_keys_recursive::<Dfs>();
            assert_eq!(&union, &expected);
        }
    }

    #[test]
    fn union_recursive_complex() {
        let base = json!({
            "title": "This is a title",
            "person" : {
                "firstName" : "John",
                "lastName" : "Doe"
            },
            "cities": [ "london", "paris" ]
        });
        let merge = json!({
            "title": "",
            "person" : {
                "firstName" : "",
                "lastName" : "",
                "new" : "this field is new"
            },
            "cities": [ "london" ],
            "new": [ "this is new" ],
        });

        let expected = json!({
            "title": "This is a title",
            "person" : {
                "firstName" : "John",
                "lastName" : "Doe",
                "new" : "this field is new"
            },
            "cities": [ "london", "paris", "london" ],
            "new": [ "this is new" ],
        });
        assert_eq!(&base.into_union_recursive::<Dfs>(&merge), &expected);
    }
}
