pub mod keys;
pub mod sortable_value;
pub mod values;

use super::iter::{Iter, Traverser};
use super::IndexPath;
use itertools::Itertools;
pub use keys::SortKeys;
use serde_json::{Map, Value};
pub use sortable_value::Ord as ValueOrd;
use std::cmp::Ordering;
pub use values::SortValues;

pub trait PartialEqOrdered {
    fn eq(&self, other: &Self) -> bool;
}

impl PartialEqOrdered for Value {
    fn eq(&self, other: &Self) -> bool {
        // iterate over all entries and check for equal values and indices
        use crate::iter::Dfs;
        let entries = self.iter_recursive::<Dfs>();
        let other_entries = other.iter_recursive::<Dfs>();
        itertools::equal(entries, other_entries)
    }
}

pub trait Sort: SortKeys + SortValues {
    fn sort(&mut self) {}
}
