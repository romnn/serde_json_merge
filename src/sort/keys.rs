use super::IndexPath;
use crate::iter::{Iter, Traverser};
use itertools::Itertools;
use serde_json::{Map, Value};
use std::cmp::Ordering;

pub trait SortKeys: Sized {
    fn sort_keys_by<F>(&mut self, cmp: &mut F)
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering;

    fn sort_keys_unstable_by<F>(&mut self, cmp: &mut F)
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering;

    fn sort_keys_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering;

    fn sort_keys_unstable_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering;

    fn sort_keys(&mut self) {
        self.sort_keys_by(&mut |ak: &IndexPath, _, bk: &IndexPath, _| Ord::cmp(&ak, &bk))
    }

    fn sort_keys_unstable(&mut self) {
        self.sort_keys_unstable_by(&mut |ak: &IndexPath, _, bk: &IndexPath, _| Ord::cmp(&ak, &bk))
    }

    fn sorted_keys(mut self) -> Self {
        self.sort_keys();
        self
    }

    fn sorted_keys_unstable(mut self) -> Self {
        self.sort_keys_unstable();
        self
    }

    fn sorted_keys_by<F>(mut self, cmp: &mut F) -> Self
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        self.sort_keys_by(cmp);
        self
    }

    fn sorted_keys_unstable_by<F>(mut self, cmp: &mut F) -> Self
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        self.sort_keys_unstable_by(cmp);
        self
    }

    fn sort_keys_recursive<T>(&mut self)
    where
        T: Traverser,
    {
        self.sort_keys_by_recursive::<T, _>(&mut |ak: &IndexPath, _, bk: &IndexPath, _| {
            Ord::cmp(&ak, &bk)
        })
    }

    fn sort_keys_unstable_recursive<T>(&mut self)
    where
        T: Traverser,
    {
        self.sort_keys_unstable_by_recursive::<T, _>(&mut |ak: &IndexPath, _, bk: &IndexPath, _| {
            Ord::cmp(&ak, &bk)
        })
    }

    fn sorted_keys_recursive<T>(mut self) -> Self
    where
        T: Traverser,
    {
        self.sort_keys_recursive::<T>();
        self
    }

    fn sorted_keys_unstable_recursive<T>(mut self) -> Self
    where
        T: Traverser,
    {
        self.sort_keys_unstable_recursive::<T>();
        self
    }

    fn sorted_keys_by_recursive<T, F>(mut self, cmp: &mut F) -> Self
    where
        T: Traverser,
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        self.sort_keys_by_recursive::<T, F>(cmp);
        self
    }

    fn sorted_keys_unstable_by_recursive<T, F>(mut self, cmp: &mut F) -> Self
    where
        T: Traverser,
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        self.sort_keys_unstable_by_recursive::<T, F>(cmp);
        self
    }
}

fn sort_cmp_wrapper<'a, 'b>(
    a: &'a (String, Value),
    b: &'b (String, Value),
    // a: &'a (&String, &mut Value),
    // b: &'b (&String, &mut Value),
    // ) -> (IndexPath, Value, IndexPath, Value) {
) -> (IndexPath, &'a Value, IndexPath, &'b Value) {
    // let (&(ak, ref av), &(bk, ref bv)) = (a, b);
    let (&(ref ak, ref av), &(ref bk, ref bv)) = (a, b);
    // clone required :(
    // not possible now because would require 'static
    let ak = IndexPath::new(ak.clone());
    let bk = IndexPath::new(bk.clone());
    (ak, av, bk, bv)
}

///
///
/// ```
/// // #[cfg(not(feature = "preserve_order"))]
/// // use alloc::collections::{btree_map, BTreeMap};
/// // #[cfg(feature = "preserve_order")]
/// // use indexmap::{self, IndexMap};
/// ```
///
/// `IndexMap` does implement sorting functions `while BTreeMap` does not.
///
/// we copy the data on a best effort
///
impl SortKeys for Map<String, Value> {
    fn sort_keys_by<F>(&mut self, mut cmp: &mut F)
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        *self = self.clone().sorted_keys_by::<F>(cmp);
    }

    fn sort_keys_unstable_by<F>(&mut self, mut cmp: &mut F)
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        *self = self.clone().sorted_keys_unstable_by::<F>(cmp);
    }

    fn sorted_keys_by<F>(mut self, mut cmp: &mut F) -> Self
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        let mut entries = Vec::from_iter(self.into_iter());
        entries.sort_by(|a, b| {
            let (ak, av, bk, bv) = sort_cmp_wrapper(a, b);
            cmp(&ak, &av, &bk, &bv)
        });
        entries.into_iter().collect()
    }

    fn sorted_keys_unstable_by<F>(mut self, mut cmp: &mut F) -> Self
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        let mut entries = Vec::from_iter(self.into_iter());
        entries.sort_unstable_by(|a, b| {
            let (ak, av, bk, bv) = sort_cmp_wrapper(a, b);
            cmp(&ak, &av, &bk, &bv)
        });
        entries.into_iter().collect()
    }

    fn sort_keys_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        *self = self.clone().sorted_keys_by::<F>(cmp);
        for (key, value) in self.iter_mut() {
            let key = IndexPath::new(key.clone());
            value.sort_keys_by_recursive::<T, F>(cmp);
        }
    }

    fn sort_keys_unstable_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        *self = self.clone().sorted_keys_unstable_by::<F>(cmp);
        for (key, value) in self.iter_mut() {
            let key = IndexPath::new(key.clone());
            value.sort_keys_unstable_by_recursive::<T, F>(cmp);
        }
    }
}

impl SortKeys for Value {
    fn sort_keys_by<F>(&mut self, mut cmp: &mut F)
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        match self {
            Value::Object(ref mut map) => {
                map.sort_keys_by(cmp);
                // let mut entries = Vec::from_iter(map.into_iter());
                // entries.sort_by(|a, b| {
                //     let (ak, av, bk, bv) = sort_cmp_wrapper(a, b);
                //     cmp(&ak, av, &bk, bv)
                // });
                // let sorted_map: Map<String, Value> = entries
                //     .into_iter()
                //     .map(|(key, value)| (key.clone(), value.clone()))
                //     .collect();
                // *map = sorted_map;
            }
            _ => {}
        }
    }

    fn sort_keys_unstable_by<F>(&mut self, mut cmp: &mut F)
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        match self {
            Value::Object(ref mut map) => {
                map.sort_keys_unstable_by(cmp);
            }
            _ => {}
        }
    }

    fn sort_keys_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        self.iter_mut_recursive::<T>()
            .for_each(|_, val: &mut Value| {
                val.sort_keys_by(cmp);
            });
    }

    fn sort_keys_unstable_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        self.iter_mut_recursive::<T>()
            .for_each(|_, val: &mut Value| {
                val.sort_keys_unstable_by(cmp);
            });
    }
}

#[cfg(feature = "preserve_order")]
#[cfg(test)]
pub mod test {
    use super::*;
    use crate::iter::Dfs;
    use crate::test::{assert_eq_ordered, assert_ne_ordered};
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    #[test]
    fn preserves_order() {
        assert_ne_ordered!(
            json!({
                "b": "b",
                "a": "a",
                "d": { "1": "1", "2": "2" },
                "c": "c",
            }),
            json!({
                "a": "a",
                "b": "b",
                "c": "c",
                "d": { "1": "1", "2": "2" },
            })
        );
    }

    #[test]
    fn sort_keys() {
        let mut value = json!({
            "a": "a",
            "c": "c",
            "b": "b",
            "d": { "2": "2", "1": "1" },
        });
        let expected = json!({
            "a": "a",
            "b": "b",
            "c": "c",
            "d": { "2": "2", "1": "1" },
        });
        assert_eq_ordered!(value.clone().sorted_keys(), &expected);
        assert_eq_ordered!(value.clone().sorted_keys_unstable(), &expected);
    }

    #[test]
    fn sort_keys_recursive() {
        let value = json!({
            "a": "a",
            "c": "c",
            "b": "b",
            "d": { "2": "2", "1": "1" },
        });
        let expected = json!({
            "a": "a",
            "b": "b",
            "c": "c",
            "d": { "1": "1", "2": "2" },
        });
        assert_eq_ordered!(value.clone().sorted_keys_recursive::<Dfs>(), &expected);
        assert_eq_ordered!(
            value.clone().sorted_keys_unstable_recursive::<Dfs>(),
            &expected
        );
    }

    #[test]
    fn sort_keys_by_recursive_custom_ordering_reversed() {
        let value = json!({
            "a": "a",
            "c": "c",
            "b": "b",
            "d": { "2": "2", "1": "1" },
        });
        let mut cmp = |ak: &IndexPath, _av: &Value, bk: &IndexPath, _bv: &Value| {
            assert!(ak.is_object_key());
            assert!(bk.is_object_key());
            Ord::cmp(bk, ak)
        };
        let expected = json!({
            "d": { "2": "2", "1": "1" },
            "c": "c",
            "b": "b",
            "a": "a",
        });
        assert_eq_ordered!(
            value.clone().sorted_keys_by_recursive::<Dfs, _>(&mut cmp),
            &expected
        );
        assert_eq_ordered!(
            value
                .clone()
                .sorted_keys_unstable_by_recursive::<Dfs, _>(&mut cmp),
            &expected
        );
    }

    #[test]
    fn sort_keys_by_recursive_custom_ordering_by_value() {
        let value = json!({
            "b": "a",
            "a": "c",
            "d": "b",
            "x": { "1": "2", "2": "1" },
        });
        let mut cmp = |ak: &IndexPath, av: &Value, bk: &IndexPath, bv: &Value| {
            assert!(ak.is_object_key());
            assert!(bk.is_object_key());
            // sort by string values, all other values are
            match (av, bv) {
                (Value::String(a), Value::String(b)) => Ord::cmp(a, b),
                (Value::String(a), _) => Ordering::Less,
                (_, Value::String(a)) => Ordering::Greater,
                _ => Ordering::Equal,
            }
        };
        let expected = json!({
            "b": "a",
            "d": "b",
            "a": "c",
            "x": { "2": "1", "1": "2" },
        });
        assert_eq_ordered!(
            value.clone().sorted_keys_by_recursive::<Dfs, _>(&mut cmp),
            &expected
        );
        assert_eq_ordered!(
            value
                .clone()
                .sorted_keys_unstable_by_recursive::<Dfs, _>(&mut cmp),
            &expected
        );
    }
}
