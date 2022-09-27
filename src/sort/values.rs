use super::sortable_value::Ord as ValueOrd;
use crate::iter::{Iter, Traverser};
use serde_json::Value;
use std::cmp::Ordering;

pub trait Sort: Sized {
    fn sort_values_by<F>(&mut self, cmp: &mut F)
    where
        F: FnMut(&Value, &Value) -> Ordering;

    fn sort_values_unstable_by<F>(&mut self, cmp: &mut F)
    where
        F: FnMut(&Value, &Value) -> Ordering;

    fn sort_values_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&Value, &Value) -> Ordering;

    fn sort_values_unstable_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&Value, &Value) -> Ordering;

    fn sort_values(&mut self) {
        self.sort_values_by(&mut |a: &Value, b: &Value| ValueOrd::cmp(a, b));
    }

    fn sort_values_unstable(&mut self) {
        self.sort_values_unstable_by(&mut |a: &Value, b: &Value| ValueOrd::cmp(a, b));
    }

    #[must_use]
    fn sorted_values(mut self) -> Self {
        self.sort_values();
        self
    }

    #[must_use]
    fn sorted_values_unstable(mut self) -> Self {
        self.sort_values_unstable();
        self
    }

    #[must_use]
    fn sorted_values_by<F>(mut self, cmp: &mut F) -> Self
    where
        F: FnMut(&Value, &Value) -> Ordering,
    {
        self.sort_values_by(cmp);
        self
    }

    #[must_use]
    fn sorted_values_unstable_by<F>(mut self, cmp: &mut F) -> Self
    where
        F: FnMut(&Value, &Value) -> Ordering,
    {
        self.sort_values_unstable_by(cmp);
        self
    }

    fn sort_values_recursive<T>(&mut self)
    where
        T: Traverser,
    {
        self.sort_values_by_recursive::<T, _>(&mut |a: &Value, b: &Value| ValueOrd::cmp(a, b));
    }

    fn sort_values_unstable_recursive<T>(&mut self)
    where
        T: Traverser,
    {
        self.sort_values_unstable_by_recursive::<T, _>(&mut |a: &Value, b: &Value| {
            ValueOrd::cmp(a, b)
        });
    }

    #[must_use]
    fn sorted_values_recursive<T>(mut self) -> Self
    where
        T: Traverser,
    {
        self.sort_values_recursive::<T>();
        self
    }

    #[must_use]
    fn sorted_values_unstable_recursive<T>(mut self) -> Self
    where
        T: Traverser,
    {
        self.sort_values_unstable_recursive::<T>();
        self
    }

    #[must_use]
    fn sorted_values_by_recursive<T, F>(mut self, cmp: &mut F) -> Self
    where
        T: Traverser,
        F: FnMut(&Value, &Value) -> Ordering,
    {
        self.sort_values_by_recursive::<T, F>(cmp);
        self
    }

    #[must_use]
    fn sorted_values_unstable_by_recursive<T, F>(mut self, cmp: &mut F) -> Self
    where
        T: Traverser,
        F: FnMut(&Value, &Value) -> Ordering,
    {
        self.sort_values_unstable_by_recursive::<T, F>(cmp);
        self
    }
}

impl Sort for Value {
    #[inline]
    fn sort_values_by<F>(&mut self, cmp: &mut F)
    where
        F: FnMut(&Value, &Value) -> Ordering,
    {
        if let Value::Array(ref mut arr) = self {
            arr.sort_by(cmp);
        }
    }

    #[inline]
    fn sort_values_unstable_by<F>(&mut self, cmp: &mut F)
    where
        F: FnMut(&Value, &Value) -> Ordering,
    {
        if let Value::Array(ref mut arr) = self {
            arr.sort_unstable_by(cmp);
        }
    }

    #[inline]
    fn sort_values_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&Value, &Value) -> Ordering,
    {
        self.mutate_recursive::<T>().for_each(|_, val: &mut Value| {
            val.sort_values_by(cmp);
        });
    }

    #[inline]
    fn sort_values_unstable_by_recursive<T, F>(&mut self, cmp: &mut F)
    where
        T: Traverser,
        F: FnMut(&Value, &Value) -> Ordering,
    {
        self.mutate_recursive::<T>().for_each(|_, val: &mut Value| {
            val.sort_values_unstable_by(cmp);
        });
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::iter::dfs::Dfs;
    use crate::sort::ValueOrd;
    use crate::test::assert_eq_ordered;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn sort_values() {
        let value = json!({
            "a": "a",
            "c": "c",
            "d": [3, 2, 1],
            "b": { "1": "2", "2": "1" },
        });
        let expected = json!({
            "a": "a",
            "c": "c",
            "d": [3, 2, 1],
            "b": { "1": "2", "2": "1" },
        });
        assert_eq!(&value.clone().sorted_values(), &expected);
        assert_eq!(&value.sorted_values_unstable(), &expected);
    }

    #[test]
    fn sort_values_recursive() {
        let value = json!({
            "a": "a",
            "c": "c",
            "d": [3, 2, 1],
            "b": { "1": "2", "2": "1" },
        });
        let expected = json!({
            "a": "a",
            "c": "c",
            "d": [1, 2, 3],
            "b": { "1": "2", "2": "1" },
        });
        assert_eq!(&value.clone().sorted_values_recursive::<Dfs>(), &expected);
        assert_eq!(&value.sorted_values_unstable_recursive::<Dfs>(), &expected);
    }

    #[test]
    fn sort_values_by_recursive_custom_ordering_reversed() {
        let value = json!({
            "a": "a",
            "c": "c",
            "b": [1, 2, 3],
            "d": { "2": "1", "1": "2" },
        });
        let mut cmp = |a: &Value, b: &Value| ValueOrd::cmp(b, a);
        let expected = json!({
            "a": "a",
            "c": "c",
            "b": [3, 2, 1],
            "d": { "2": "1", "1": "2" },
        });
        assert_eq_ordered!(
            value.clone().sorted_values_by_recursive::<Dfs, _>(&mut cmp),
            &expected
        );
        assert_eq_ordered!(
            value.sorted_values_unstable_by_recursive::<Dfs, _>(&mut cmp),
            &expected
        );
    }

    #[test]
    fn sort_values_by_recursive_custom_ordering_by_value() {
        let value = json!({
            "x": [ ["b", "b"], ["c"], ["a", "a", "a"], false, "test" ],
        });
        let expected_default = json!({
            "x": [ false, "test", ["a", "a", "a"], ["b", "b"], ["c"] ],
        });
        assert_eq!(
            &value.clone().sorted_values_recursive::<Dfs>(),
            &expected_default
        );
        assert_eq!(
            &value.clone().sorted_values_unstable_recursive::<Dfs>(),
            &expected_default
        );

        let mut cmp = |a: &Value, b: &Value| {
            // sort arrays by length
            match (a, b) {
                (Value::Array(a), Value::Array(b)) => Ord::cmp(&a.len(), &b.len()),
                _ => ValueOrd::cmp(a, b),
            }
        };
        let expected_custom_cmp = json!({
            "x": [ false, "test", ["c"], ["b", "b"], ["a", "a", "a"] ],
        });
        assert_eq!(
            &value.clone().sorted_values_by_recursive::<Dfs, _>(&mut cmp),
            &expected_custom_cmp
        );
        assert_eq!(
            &value.sorted_values_unstable_by_recursive::<Dfs, _>(&mut cmp),
            &expected_custom_cmp
        );
    }
}
