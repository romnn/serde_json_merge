//use super::iter::{Iter, Traverser};
//use super::IndexPath;
//use itertools::Itertools;
//use serde_json::{Map, Value};
//use std::cmp::Ordering;

//pub trait Sort {
//    // : SortInPlace {
//    // /// Method use to merge two Json Values : ValueA <- ValueB.
//    // fn merge(&mut self, other: Value);
//    // /// Merge a new value in specific json pointer. If the field can't be merge in the specific
//    // /// path, it raise an error.
//    // fn merge_in(&mut self, json_pointer: &str, new_json_value: Value) -> io::Result<()>;
//    // sort (rec)
//    // sort values (rec)
//    // sort keys (rec)
//    // sorted (rec)
//    // sorted keys (rec)
//    // sorted values (rec)
//    // fn sorted(self) -> Self;
//    // fn eq_ordered(&self, other: &Self) -> bool;
//    fn eq(&self, other: &Self) -> bool;

//    fn sort_keys(&mut self);

//    fn sorted_keys(self) -> Self;

//    fn sort_keys_by<F>(&mut self, cmp: &mut F)
//    where
//        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering;

//    fn sorted_keys_by<F>(self, cmp: &mut F) -> Self
//    where
//        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering;

//    fn sort_keys_recursive<T>(&mut self)
//    where
//        T: Traverser;

//    fn sorted_keys_recursive<T>(self) -> Self
//    where
//        T: Traverser;

//    fn sort_keys_by_recursive<T, F>(&mut self, cmp: &mut F)
//    where
//        T: Traverser,
//        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering;

//    fn sorted_keys_by_recursive<T, F>(self, cmp: &mut F) -> Self
//    where
//        T: Traverser,
//        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering;

//    fn sort(&mut self);
//    // fn sort_by(&mut self);
//    fn sort_values(&mut self);

//    // F: FnMut(&(&String, &Value), &(&String, &Value)) -> Ordering;

//    // fn sort_values_by(&mut self);

//    // fn sort_recursive(&mut self);
//    // fn sort_keys_recursive(&mut self);
//    // fn sort_values_recursive(&mut self);
//}

/////
/////
///// ```
///// // #[cfg(not(feature = "preserve_order"))]
///// // use alloc::collections::{btree_map, BTreeMap};
///// // #[cfg(feature = "preserve_order")]
///// // use indexmap::{self, IndexMap};
///// ```
/////
///// `IndexMap` does implement sorting functions `while BTreeMap` does not.
/////
///// we copy the data on a best effort
/////

//impl Sort for Value {
//    fn eq(&self, other: &Self) -> bool {
//        // iterate over all entries and check for equal values and indices
//        use crate::iter::Dfs;
//        let entries = self.iter_recursive::<Dfs>();
//        let other_entries = other.iter_recursive::<Dfs>();
//        itertools::equal(entries, other_entries)
//    }

//    fn sort(&mut self) {}

//    fn sort_keys_by<F>(&mut self, mut cmp: &mut F)
//    where
//        F: FnMut(&IndexPath, &Self, &IndexPath, &Self) -> Ordering,
//    {
//        match self {
//            Value::Object(ref mut map) => {
//                let sorted_map: Map<String, Value> = map
//                    .into_iter()
//                    .sorted_by(|a, b| {
//                        let (&(ak, ref av), &(bk, ref bv)) = (a, b);
//                        // clone required :(
//                        // not possible now because would require 'static
//                        let ak = IndexPath::new(ak.clone());
//                        let bk = IndexPath::new(bk.clone());
//                        cmp(&ak, av, &bk, bv)
//                    })
//                    .map(|(key, value)| (key.clone(), value.clone()))
//                    .collect();
//                *map = sorted_map;
//            }
//            _ => {}
//        }
//    }

//    fn sort_keys(&mut self) {
//        self.sort_keys_by(&mut |ak: &IndexPath, _, bk: &IndexPath, _| Ord::cmp(&ak, &bk))
//    }

//    fn sorted_keys(mut self) -> Self {
//        self.sort_keys();
//        self
//    }

//    fn sorted_keys_by<F>(mut self, cmp: &mut F) -> Self
//    where
//        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
//    {
//        self.sort_keys_by(cmp);
//        self
//    }

//    fn sort_keys_by_recursive<T, F>(&mut self, cmp: &mut F)
//    where
//        T: Traverser,
//        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
//    {
//        self.iter_mut_recursive::<T>()
//            .for_each(|_, val: &mut Value| {
//                dbg!(&val);
//                val.sort_keys_by(cmp);
//                dbg!(&val);
//            });
//    }

//    fn sort_keys_recursive<T>(&mut self)
//    where
//        T: Traverser,
//    {
//        self.sort_keys_by_recursive::<T, _>(&mut |ak: &IndexPath, _, bk: &IndexPath, _| {
//            Ord::cmp(&ak, &bk)
//        })
//    }

//    fn sorted_keys_recursive<T>(mut self) -> Self
//    where
//        T: Traverser,
//    {
//        self.sort_keys_recursive::<T>();
//        self
//    }

//    fn sorted_keys_by_recursive<T, F>(mut self, cmp: &mut F) -> Self
//    where
//        T: Traverser,
//        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
//    {
//        self.sort_keys_by_recursive::<T, F>(cmp);
//        self
//    }

//    fn sort_values(&mut self) {}

//    // fn sort_recursive(&mut self);
//    // fn sort_keys_recursive(&mut self);
//    // fn sort_values_recursive(&mut self);
//}

//#[cfg(test)]
//pub mod test {
//    use super::*;
//    use crate::iter::Dfs;
//    use crate::test::{assert_eq_ordered, assert_ne_ordered};
//    use anyhow::Result;
//    use pretty_assertions::assert_eq;
//    use serde_json::{json, Value};

//    #[cfg(feature = "preserve_order")]
//    #[test]
//    fn preserves_order() {
//        assert_ne_ordered!(
//            json!({
//                "b": "b",
//                "a": "a",
//                "d": { "1": "1", "2": "2" },
//                "c": "c",
//            }),
//            json!({
//                "a": "a",
//                "b": "b",
//                "c": "c",
//                "d": { "1": "1", "2": "2" },
//            })
//        );
//    }

//    #[test]
//    fn sort_keys() -> Result<()> {
//        let mut value = json!({
//            "a": "a",
//            "c": "c",
//            "b": "b",
//            "d": { "2": "2", "1": "1" },
//        });
//        assert_eq_ordered!(
//            value.sorted_keys(),
//            json!({
//                "a": "a",
//                "b": "b",
//                "c": "c",
//                "d": { "2": "2", "1": "1" },
//            })
//        );
//        Ok(())
//    }

//    #[test]
//    fn sort_keys_recursive() -> Result<()> {
//        let value = json!({
//            "a": "a",
//            "c": "c",
//            "b": "b",
//            "d": { "2": "2", "1": "1" },
//        });
//        assert_eq_ordered!(
//            value.sorted_keys_recursive::<Dfs>(),
//            json!({
//                "a": "a",
//                "b": "b",
//                "c": "c",
//                "d": { "1": "1", "2": "2" },
//            })
//        );
//        Ok(())
//    }

//    #[test]
//    fn sort_keys_by_recursive_custom_ordering_reversed() -> Result<()> {
//        let value = json!({
//            "a": "a",
//            "c": "c",
//            "b": "b",
//            "d": { "2": "2", "1": "1" },
//        });
//        assert_eq_ordered!(
//            value.sorted_keys_by_recursive::<Dfs, _>(
//                &mut |ak: &IndexPath, _, bk: &IndexPath, _| {
//                    assert!(ak.is_object_key());
//                    assert!(bk.is_object_key());
//                    Ord::cmp(&bk, &ak)
//                }
//            ),
//            json!({
//                "d": { "2": "2", "1": "1" },
//                "c": "c",
//                "b": "b",
//                "a": "a",
//            })
//        );
//        Ok(())
//    }

//    #[test]
//    fn sort_keys_by_recursive_custom_ordering_by_value() -> Result<()> {
//        let value = json!({
//            "b": "a",
//            "a": "c",
//            "d": "b",
//            "x": { "1": "2", "2": "1" },
//        });
//        assert_eq_ordered!(
//            value.sorted_keys_by_recursive::<Dfs, _>(
//                &mut |ak: &IndexPath, av: &Value, bk: &IndexPath, bv: &Value| {
//                    assert!(ak.is_object_key());
//                    assert!(bk.is_object_key());
//                    // sort by string values, all other values are
//                    match (av, bv) {
//                        (Value::String(a), Value::String(b)) => Ord::cmp(a, b),
//                        (Value::String(a), _) => Ordering::Less,
//                        (_, Value::String(a)) => Ordering::Greater,
//                        _ => Ordering::Equal,
//                    }
//                    // Ord::cmp(&ak, &bk)
//                }
//            ),
//            json!({
//                "b": "a",
//                "d": "b",
//                "a": "c",
//                "x": { "2": "1", "1": "2" },
//            })
//        );
//        Ok(())
//    }
//}
