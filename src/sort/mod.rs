pub mod keys;
pub mod sortable_value;
pub mod values;

use super::index::Path as IndexPath;
use super::iter::{dfs::Dfs, Iter, Traverser};
pub use keys::Sort as Keys;
use serde_json::Value;
pub use sortable_value::Ord as ValueOrd;
pub use values::Sort as Values;

/// Ordered `PartialEq`
///
/// Iterates over all entries and check for equal values and indices
///
pub trait PartialEqOrdered {
    fn eq(&self, other: &Self) -> bool;
}

impl PartialEqOrdered for Value {
    fn eq(&self, other: &Self) -> bool {
        let entries = self.iter_recursive::<Dfs>();
        let other_entries = other.iter_recursive::<Dfs>();
        entries.into_iter().eq(other_entries)
    }
}

impl<'a> PartialEqOrdered for &'a Value {
    fn eq(&self, other: &Self) -> bool {
        PartialEqOrdered::eq(*self, *other)
    }
}

pub trait Sort: Keys + Values {
    #[inline]
    fn sort(&mut self) {
        self.sort_keys();
        self.sort_values();
    }

    #[inline]
    fn sort_recursive<T>(&mut self)
    where
        T: Traverser,
    {
        self.sort_keys_recursive::<T>();
        self.sort_values_recursive::<T>();
    }

    #[inline]
    fn sort_unstable(&mut self) {
        self.sort_keys_unstable();
        self.sort_values_unstable();
    }

    #[inline]
    fn sort_unstable_recursive<T>(&mut self)
    where
        T: Traverser,
    {
        self.sort_keys_unstable_recursive::<T>();
        self.sort_values_unstable_recursive::<T>();
    }

    #[inline]
    #[must_use]
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }

    #[inline]
    #[must_use]
    fn sorted_recursive<T>(mut self) -> Self
    where
        T: Traverser,
    {
        self.sort_recursive::<T>();
        self
    }

    #[inline]
    #[must_use]
    fn sorted_unstable(mut self) -> Self {
        self.sort_unstable();
        self
    }

    #[inline]
    #[must_use]
    fn sorted_unstable_recursive<T>(mut self) -> Self
    where
        T: Traverser,
    {
        self.sort_unstable_recursive::<T>();
        self
    }
}

impl Sort for Value {}

#[cfg(test)]
pub mod test {
    use crate::test::{assert_eq_ordered, assert_ne_ordered};
    use serde_json::json;

    #[cfg(feature = "preserve_order")]
    #[test]
    fn preserves_order() {
        let value = json!({
            "b": "b",
            "a": "a",
            "d": { "1": "1", "2": "2" },
            "c": "c",
        });
        assert_ne_ordered!(
            &value,
            &json!({
                "a": "a",
                "b": "b",
                "c": "c",
                "d": { "1": "1", "2": "2" },
            })
        );
        assert_ne_ordered!(
            &value,
            &json!({
                "b": "b",
                "a": "a",
                "d": { "2": "2", "1": "1" },
                "c": "c",
            })
        );
        assert_eq_ordered!(&value, &value,);
    }
}
