use crate::index::{Index, Path as IndexPath};
use crate::iter::{Dfs, Iter, Traverser};
use serde_json::{Map, Value};
use std::borrow::Borrow;

pub trait Merge: Sized {
    fn merge<T>(&mut self, other: &Self)
    where
        T: Traverser;

    fn merge_recursive<T>(&mut self, other: &Self)
    where
        T: Traverser;

    fn merge_by<T, F>(&mut self, other: &Self, merge: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool;

    fn merge_by_recursive<T, F>(&mut self, other: &Self, merge: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool;

    #[inline]
    fn merged<T>(mut self, other: &Self) -> Self
    where
        T: Traverser,
    {
        self.merge::<T>(other);
        self
    }

    #[inline]
    fn merged_recursive<T>(mut self, other: &Self) -> Self
    where
        T: Traverser,
    {
        self.merge_recursive::<T>(other);
        self
    }

    #[inline]
    fn merged_by<T, F>(mut self, other: &Self, merge: &mut F) -> Self
    where
        T: Traverser,
        F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool,
    {
        self.merge_by::<T, F>(other, merge);
        self
    }

    #[inline]
    fn merged_by_recursive<T, F>(mut self, other: &Self, merge: &mut F) -> Self
    where
        T: Traverser,
        F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool,
    {
        self.merge_by_recursive::<T, F>(other, merge);
        self
    }
}

impl Merge for Value {
    #[inline]
    fn merge<T>(&mut self, other: &Self)
    where
        T: Traverser,
    {
        self.merge_by::<T, _>(other, &mut merge_func);
    }

    #[inline]
    fn merge_recursive<T>(&mut self, other: &Self)
    where
        T: Traverser,
    {
        self.merge_by_recursive::<T, _>(other, &mut merge_func);
    }

    #[inline]
    fn merge_by<T, F>(&mut self, other: &Self, merge: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool,
    {
        let mut traverser = T::new();
        traverser.set_limit(None);
        traverser.set_depth(1);
        while let Some(_) = traverser.process_next(other, |idx, new_value| {
            if let Some(value) = self.get_index_mut(idx) {
                merge(idx, value, new_value)
            } else {
                true
            }
        }) {}
    }

    #[inline]
    fn merge_by_recursive<T, F>(&mut self, other: &Self, merge: &mut F)
    where
        T: Traverser,
        F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool,
    {
        let mut traverser = T::new();
        traverser.set_limit(None);
        traverser.set_depth(None);
        while let Some(_) = traverser.process_next(other, |idx, new_value| {
            if let Some(value) = self.get_index_mut(idx) {
                merge(idx, value, new_value)
            } else {
                true
            }
        }) {}
    }
}

fn merge_func(idx: &IndexPath, this: &mut Value, other: Option<&Value>) -> bool {
    match (this, other) {
        // add new fields when merging two objects
        (&mut Value::Object(ref mut this), Some(&Value::Object(ref other))) => {
            for (k, v) in other {
                this.entry(k.clone()).or_insert(Value::Null);
            }
            true
        }
        // extend array with other array
        (&mut Value::Array(ref mut this), Some(&Value::Array(ref other))) => {
            this.extend(other.clone());
            false
        }
        // extend array with other value
        (&mut Value::Array(ref mut this), Some(other)) => {
            this.extend([other.clone()]);
            false
        }
        // do not overwrite anything with null
        (_, Some(&Value::Null)) => false,
        // overwrite this with other
        (this, Some(other)) => {
            *this = other.clone();
            false
        }
        _ => false,
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    #[test]
    fn merge_array_string() {
        let base = json!(["a", "b"]);
        let merge = json!(["b", "c"]);
        assert_eq!(
            &base.merged_recursive::<Dfs>(&merge),
            &json!(["a", "b", "b", "c"])
        );
    }

    #[test]
    fn merge_array_object() {
        let base = json!([{"value": "a"}, {"value": "b"}]);
        let merge = json!([{"value": "b"}, {"value": "c"}]);
        dbg!(base.clone().merged_recursive::<Dfs>(&merge));
        assert_eq!(
            &base.merged_recursive::<Dfs>(&merge),
            &json!([
                {"value": "a"},
                {"value": "b"},
                {"value": "b"},
                {"value": "c"}
            ])
        );
    }

    #[test]
    fn merge_object() {
        let base = json!({"value1": "a", "value2": "b"});
        let merge = json!({"value1": "a", "value2": "c", "value3": "d"});
        assert_eq!(
            &base.merged_recursive::<Dfs>(&merge),
            &json!({
                "value1": "a",
                "value2": "c",
                "value3": "d",
            })
        );
    }

    #[test]
    fn merge_string() {
        let base = json!("a");
        let merge = json!("b");
        assert_eq!(&base.merged_recursive::<Dfs>(&merge), &merge);
    }
}
