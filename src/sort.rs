use itertools::Itertools;
use serde_json::{Map, Value};
use std::cmp::Ordering;

pub trait SortKeyComparator: FnMut(&(&String, &Value), &(&String, &Value)) -> Ordering {}

pub trait Sort {
    // : SortInPlace {
    // /// Method use to merge two Json Values : ValueA <- ValueB.
    // fn merge(&mut self, other: Value);
    // /// Merge a new value in specific json pointer. If the field can't be merge in the specific
    // /// path, it raise an error.
    // fn merge_in(&mut self, json_pointer: &str, new_json_value: Value) -> io::Result<()>;
    // sort (rec)
    // sort values (rec)
    // sort keys (rec)
    // sorted (rec)
    // sorted keys (rec)
    // sorted values (rec)
    // fn sorted(self) -> Self;

    fn sorted_keys(self) -> Self;

    fn sort(&mut self);
    fn sort_keys(&mut self);
    fn sort_values(&mut self);

    // fn sort_by(&mut self);
    fn sort_keys_by<F>(&mut self, cmp: F)
    where
        F: FnMut(&(&String, &Value), &(&String, &Value)) -> Ordering;

    // fn sort_values_by(&mut self);

    // fn sort_recursive(&mut self);
    // fn sort_keys_recursive(&mut self);
    // fn sort_values_recursive(&mut self);
}

///
///
/// ```
/// #[cfg(not(feature = "preserve_order"))]
/// use alloc::collections::{btree_map, BTreeMap};
/// #[cfg(feature = "preserve_order")]
/// use indexmap::{self, IndexMap};
/// ```
///
/// `IndexMap` does implement sorting functions `while BTreeMap` does not.
///
/// we copy the data on a best effort
///

impl Sort for serde_json::Value {
    fn sort(&mut self) {}

    fn sort_keys_by<F>(&mut self, cmp: F)
    where
        F: FnMut(&(&String, &Value), &(&String, &Value)) -> Ordering,
    {
        match self {
            Value::Object(ref mut map) => {
                let sorted_map: Map<String, Value> = map
                    .iter()
                    .sorted_by(cmp) // |a, b| )
                    // .sorted_by(|key, value| cmp(key, value)) // |a, b| Ord::cmp(&b.0, &a.0))
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect();
                *map = sorted_map;
            }
            _ => {}
        }
    }

    fn sort_keys(&mut self) {
        self.sort_keys_by(|a: &(&String, &Value), b: &(&String, &Value)| Ord::cmp(&b.0, &a.0))
    }

    fn sorted_keys(mut self) -> Self {
        self.sort_keys();
        self
    }

    fn sort_values(&mut self) {}

    // fn sort_recursive(&mut self);
    // fn sort_keys_recursive(&mut self);
    // fn sort_values_recursive(&mut self);
}

// fn sort_json_value_iter(value: Value) -> Value {
//     let mut result = value.clone();
//     for idx in value.iter_indices() {
//         match value.get_path(&idx) {
//             Some(Value::Object(v)) => {
//                 let sorted: Vec<(String, Value)> = v
//                     .iter()
//                     .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
//                     .map(|(key, value)| (key.clone(), value.clone()))
//                     .collect();
//                 match result.get_path_mut(&idx) {
//                     Some(Value::Object(ref mut r)) => {
//                         *r = Map::from_iter(sorted);
//                     }
//                     _ => {}
//                 }
//             }
//             _ => {}
//         }
//     }
//     result
// }

// fn sort_json_value_rec(value: &mut Value) {
//     match value {
//         &mut Value::Object(ref mut a) => {
//             let sorted: Vec<(String, Value)> = a
//                 .iter_mut()
//                 .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
//                 .map(|(key, value)| {
//                     sort_json_value_rec(value);
//                     (key.clone(), value.clone())
//                 })
//                 .collect();
//             *a = Map::from_iter(sorted);
//         }
//         _ => {}
//     }
// }

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    // macro_rules! remove_rec {
    //     ( $value:expr, $depth:expr ) => {{
    //         let mut tmp = $value;
    //         let mut dfs_mut = DfsIterMut::new(&mut tmp).depth($depth);
    //         dfs_mut.for_each(remove_entries);
    //         tmp.clone()
    //     }};
    // }

    #[test]
    fn test_sort_json_keys_recursively() -> Result<()> {
        let mut value = json!({
            "a": "a",
            "c": "c",
            "b": "b",
            "d": { "2": "2", "1": "1" },
        });
        assert_eq!(
            value.sorted_keys(),
            json!({
                "a": "a",
                "b": "b",
                "c": "c",
                "d": { "2": "2", "1": "1" },
            })
        );
        Ok(())
    }
}
