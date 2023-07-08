use super::IndexPath;
use crate::iter::{Iter, Traverser};
use serde_json::{Map, Value};
use std::cmp::Ordering;

pub trait Sort: Sized {
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
        self.sort_keys_by(&mut |ak: &IndexPath, _, bk: &IndexPath, _| Ord::cmp(&ak, &bk));
    }

    fn sort_keys_unstable(&mut self) {
        self.sort_keys_unstable_by(&mut |ak: &IndexPath, _, bk: &IndexPath, _| Ord::cmp(&ak, &bk));
    }

    #[must_use]
    fn sorted_keys(mut self) -> Self {
        self.sort_keys();
        self
    }

    #[must_use]
    fn sorted_keys_unstable(mut self) -> Self {
        self.sort_keys_unstable();
        self
    }

    #[must_use]
    fn sorted_keys_by<F>(mut self, cmp: &mut F) -> Self
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        self.sort_keys_by(cmp);
        self
    }

    #[must_use]
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
        });
    }

    fn sort_keys_unstable_recursive<T>(&mut self)
    where
        T: Traverser,
    {
        self.sort_keys_unstable_by_recursive::<T, _>(
            &mut |ak: &IndexPath, _, bk: &IndexPath, _| Ord::cmp(&ak, &bk),
        );
    }

    #[must_use]
    fn sorted_keys_recursive<T>(mut self) -> Self
    where
        T: Traverser,
    {
        self.sort_keys_recursive::<T>();
        self
    }

    #[must_use]
    fn sorted_keys_unstable_recursive<T>(mut self) -> Self
    where
        T: Traverser,
    {
        self.sort_keys_unstable_recursive::<T>();
        self
    }

    #[must_use]
    fn sorted_keys_by_recursive<T, F>(mut self, cmp: &mut F) -> Self
    where
        T: Traverser,
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        self.sort_keys_by_recursive::<T, F>(cmp);
        self
    }

    #[must_use]
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
) -> (IndexPath, &'a Value, IndexPath, &'b Value) {
    let ((ak, av), (bk, bv)) = (a, b);
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
impl Sort for Map<String, Value> {
    fn sort_keys_by<F>(&mut self, cmp: &mut F)
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        *self = self.clone().sorted_keys_by::<F>(cmp);
    }

    fn sort_keys_unstable_by<F>(&mut self, cmp: &mut F)
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        *self = self.clone().sorted_keys_unstable_by::<F>(cmp);
    }

    #[must_use]
    fn sorted_keys_by<F>(self, cmp: &mut F) -> Self
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        let mut entries: Vec<_> = self.into_iter().collect();
        entries.sort_by(|a, b| {
            let (ak, av, bk, bv) = sort_cmp_wrapper(a, b);
            cmp(&ak, av, &bk, bv)
        });
        entries.into_iter().collect()
    }

    #[must_use]
    fn sorted_keys_unstable_by<F>(self, cmp: &mut F) -> Self
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        let mut entries: Vec<_> = self.into_iter().collect();
        entries.sort_unstable_by(|a, b| {
            let (ak, av, bk, bv) = sort_cmp_wrapper(a, b);
            cmp(&ak, av, &bk, bv)
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
            let idx = IndexPath::new(key.clone());
            value.sort_keys_by_recursive::<T, _>(&mut |ak, av, bk, bv| {
                let ak = idx.clone().join(ak);
                let bk = idx.clone().join(bk);
                cmp(&ak, av, &bk, bv)
            });
        }
    }

    fn sort_keys_unstable_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        *self = self.clone().sorted_keys_unstable_by::<F>(cmp);
        for (key, value) in self.iter_mut() {
            let idx = IndexPath::new(key.clone());
            value.sort_keys_unstable_by_recursive::<T, _>(&mut |ak, av, bk, bv| {
                let ak = idx.clone().join(ak);
                let bk = idx.clone().join(bk);
                cmp(&ak, av, &bk, bv)
            });
        }
    }
}

impl Sort for Value {
    fn sort_keys_by<F>(&mut self, cmp: &mut F)
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        if let Value::Object(ref mut map) = self {
            map.sort_keys_by(cmp);
        }
    }

    fn sort_keys_unstable_by<F>(&mut self, cmp: &mut F)
    where
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        if let Value::Object(ref mut map) = self {
            map.sort_keys_unstable_by(cmp);
        }
    }

    fn sort_keys_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        self.mutate_recursive::<T>()
            .for_each(|idx: &IndexPath, val: &mut Value| {
                val.sort_keys_by(&mut |ak, av, bk, bv| {
                    let ak = idx.clone().join(ak);
                    let bk = idx.clone().join(bk);
                    cmp(&ak, av, &bk, bv)
                });
            });
    }

    fn sort_keys_unstable_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &Value, &IndexPath, &Value) -> Ordering,
    {
        self.mutate_recursive::<T>()
            .for_each(|idx: &IndexPath, val: &mut Value| {
                val.sort_keys_unstable_by(&mut |ak, av, bk, bv| {
                    let ak = idx.clone().join(ak);
                    let bk = idx.clone().join(bk);
                    cmp(&ak, av, &bk, bv)
                });
            });
    }
}

#[cfg(feature = "preserve_order")]
#[cfg(test)]
pub mod test {
    use super::*;
    use crate::index;
    use crate::iter::dfs::Dfs;
    use crate::test::assert_eq_ordered;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    #[test]
    fn sort_keys() {
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
            "d": { "2": "2", "1": "1" },
        });
        assert_eq_ordered!(&value.clone().sorted_keys(), &expected);
        assert_eq_ordered!(&value.sorted_keys_unstable(), &expected);
    }

    #[test]
    fn sort_keys_by_uses_correct_indices() {
        use std::collections::HashSet;
        use std::{cell::RefCell, rc::Rc};

        let mut value = json!({
            "a": "a",
            "c": "c",
            "b": "b",
            "d": { "2": "2", "1": "1" },
        });
        let expected: HashSet<IndexPath> =
            HashSet::from_iter([index!("a"), index!("c"), index!("b"), index!("d")]);
        let indices = Rc::new(RefCell::new(HashSet::new()));
        let mut cmp = |ak: &IndexPath, _av: &Value, bk: &IndexPath, _bv: &Value| {
            indices.borrow_mut().extend([ak.clone(), bk.clone()]);
            Ord::cmp(ak, bk)
        };

        value.clone().sort_keys_by(&mut cmp);
        assert_eq!(&*indices.borrow(), &expected);
        indices.borrow_mut().clear();
        value.sort_keys_unstable_by(&mut cmp);
        assert_eq!(&*indices.borrow(), &expected);
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
        assert_eq_ordered!(&value.clone().sorted_keys_recursive::<Dfs>(), &expected);
        assert_eq_ordered!(&value.sorted_keys_unstable_recursive::<Dfs>(), &expected);
    }

    #[test]
    fn sort_keys_by_recursive_uses_correct_indices() {
        use std::collections::HashSet;
        use std::{cell::RefCell, rc::Rc};

        let mut value = json!({
            "a": "a",
            "c": "c",
            "b": "b",
            "d": { "2": "2", "1": "1" },
        });
        let expected: HashSet<IndexPath> = HashSet::from_iter([
            index!("a"),
            index!("c"),
            index!("b"),
            index!("d"),
            index!("d", "2"),
            index!("d", "1"),
        ]);
        let indices = Rc::new(RefCell::new(HashSet::new()));
        let mut cmp = |ak: &IndexPath, _av: &Value, bk: &IndexPath, _bv: &Value| {
            indices.borrow_mut().extend([ak.clone(), bk.clone()]);
            Ord::cmp(ak, bk)
        };

        value.clone().sort_keys_by_recursive::<Dfs, _>(&mut cmp);
        assert_eq!(&*indices.borrow(), &expected);
        indices.borrow_mut().clear();
        value.sort_keys_unstable_by_recursive::<Dfs, _>(&mut cmp);
        assert_eq!(&*indices.borrow(), &expected);
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
            &value.clone().sorted_keys_by_recursive::<Dfs, _>(&mut cmp),
            &expected
        );
        assert_eq_ordered!(
            &value.sorted_keys_unstable_by_recursive::<Dfs, _>(&mut cmp),
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
            // sort by string values
            match (av, bv) {
                (Value::String(a), Value::String(b)) => Ord::cmp(a, b),
                (Value::String(_), _) => Ordering::Less,
                (_, Value::String(_)) => Ordering::Greater,
                _ => unreachable!(),
            }
        };
        let expected = json!({
            "b": "a",
            "d": "b",
            "a": "c",
            "x": { "2": "1", "1": "2" },
        });
        assert_eq_ordered!(
            &value.clone().sorted_keys_by_recursive::<Dfs, _>(&mut cmp),
            &expected
        );
        assert_eq_ordered!(
            &value.sorted_keys_unstable_by_recursive::<Dfs, _>(&mut cmp),
            &expected
        );
    }
}
