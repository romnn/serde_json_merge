use super::{KeyValueIter, KeyValueMutator, Traverser};
use crate::{Index, IndexPath};
use serde_json::Value;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Dfs {
    queue: VecDeque<(usize, IndexPath)>,
    depth: Option<usize>,
    limit: Option<usize>,
    num_visited: usize,
}

impl Default for Dfs {
    #[inline]
    fn default() -> Self {
        Self {
            queue: VecDeque::from_iter([(0, IndexPath::empty())]),
            depth: None,
            limit: None,
            num_visited: 0,
        }
    }
}

// #[cfg(feature = "rayon")]
// impl super::ParallelTraverser for Dfs {
//     #[inline]
//     fn split(&mut self) -> Option<Self> {
//         let len = self.queue.len();
//         if len >= 2 {
//             let split = self.queue.split_off(len / 2);
//             Some(Self {
//                 queue: split,
//                 ..*self
//             })
//         } else {
//             None
//         }
//     }
// }

impl Traverser for Dfs {
    #[inline]
    fn new() -> Self {
        Self::default()
    }

    #[inline]
    fn set_limit<L>(&mut self, limit: L)
    where
        L: Into<Option<usize>>,
    {
        self.limit = limit.into();
    }

    #[inline]
    fn set_depth<D>(&mut self, depth: D)
    where
        D: Into<Option<usize>>,
    {
        self.depth = depth.into();
    }

    #[inline]
    fn reset(&mut self) {
        self.queue.clear();
        self.queue.push_back((0, IndexPath::empty()));
        self.num_visited = 0;
    }

    #[inline]
    fn mutate_then_next<'b>(
        &mut self,
        value: &mut Value,
        mut mutate: impl FnMut(&IndexPath, &mut Value),
    ) -> Option<IndexPath> {
        match self.queue.pop_back() {
            Some((depth, index)) => {
                // check if limit is reached
                self.num_visited += 1;
                if self.limit.map_or(false, |l| self.num_visited > l) {
                    return None;
                }

                // mutate before adding children to queue
                if let Some(val) = value.get_index_mut(&index) {
                    mutate(&index, val);
                }
                if self.depth.map_or(true, |d| depth < d) {
                    // add children
                    match value.get_index(&index) {
                        Some(Value::Object(o)) => {
                            self.queue.extend(o.keys().map(|key| {
                                let mut index = index.clone();
                                index.add(key.clone());
                                (depth + 1, index)
                            }));
                        }
                        Some(Value::Array(arr)) => {
                            self.queue
                                .extend(arr.iter().enumerate().rev().map(|(arr_idx, _)| {
                                    let mut index = index.clone();
                                    index.add(arr_idx);
                                    (depth + 1, index)
                                }));
                        }
                        _ => {}
                    }
                }
                Some(index)
            }
            None => None,
        }
    }

    #[inline]
    fn process_next(
        &mut self,
        root: &Value,
        mut process: impl FnMut(&IndexPath, Option<&Value>) -> bool,
    ) -> Option<IndexPath> {
        match self.queue.pop_back() {
            Some((depth, index)) => {
                // check if limit is reached
                self.num_visited += 1;
                if self.limit.map_or(false, |l| self.num_visited > l) {
                    return None;
                }

                let value = root.get_index(&index);
                let proceed = process(&index, value);
                if proceed && self.depth.map_or(true, |d| depth < d) {
                    // add children
                    match value {
                        Some(Value::Object(map)) => {
                            self.queue.extend(map.keys().rev().map(|key| {
                                let mut index = index.clone();
                                index.add(key.clone());
                                (depth + 1, index)
                            }));
                        }
                        Some(Value::Array(arr)) => {
                            self.queue
                                .extend(arr.iter().enumerate().rev().map(|(arr_idx, _)| {
                                    let mut index = index.clone();
                                    index.add(arr_idx);
                                    (depth + 1, index)
                                }));
                        }
                        _ => {}
                    }
                }
                Some(index)
            }
            None => None,
        }
    }

    #[inline]
    fn next(&mut self, value: &Value) -> Option<IndexPath> {
        self.process_next(value, |_, _| true)
    }
}

#[allow(clippy::module_name_repetitions)]
#[derive(Clone)]
pub struct Iter<'a>(KeyValueIter<'a, Dfs>);

impl<'a> Iter<'a> {
    #[inline]
    #[must_use]
    pub fn new(value: &'a Value) -> Self {
        let traverser = Dfs::default();
        Self(KeyValueIter {
            inner: value,
            traverser,
        })
    }

    #[inline]
    #[must_use]
    pub fn depth(mut self, depth: impl Into<Option<usize>>) -> Self {
        self.0.traverser.set_depth(depth);
        self
    }

    #[inline]
    #[must_use]
    pub fn limit(mut self, limit: impl Into<Option<usize>>) -> Self {
        self.0.traverser.set_limit(limit);
        self
    }
}

impl<'a> IntoIterator for Iter<'a> {
    type Item = <KeyValueIter<'a, Dfs> as Iterator>::Item;
    type IntoIter = KeyValueIter<'a, Dfs>;

    fn into_iter(self) -> Self::IntoIter {
        self.0
    }
}

pub struct IterMut<'a>(KeyValueMutator<'a, Dfs>);

impl<'a> IterMut<'a> {
    #[inline]
    #[must_use]
    pub fn new(value: &'a mut Value) -> Self {
        let traverser = Dfs::default();
        Self(KeyValueMutator {
            inner: value,
            traverser,
        })
    }

    #[inline]
    #[must_use]
    pub fn depth(mut self, depth: impl Into<Option<usize>>) -> Self {
        self.0.traverser.set_depth(depth);
        self
    }

    #[inline]
    #[must_use]
    pub fn limit(mut self, limit: impl Into<Option<usize>>) -> Self {
        self.0.traverser.set_limit(limit);
        self
    }
}

impl<'a> std::ops::Deref for IterMut<'a> {
    type Target = KeyValueMutator<'a, Dfs>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for IterMut<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::index;
    use crate::test::CollectCloned;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    macro_rules! iter_rec {
        ( $value:expr, $depth:expr ) => {{
            let tmp = $value;
            let dfs = Iter::new(&tmp).depth($depth);
            dfs.collect_cloned()
        }};
    }

    #[test]
    fn terminal_value_iter_recursive_dfs() {
        assert_eq!(iter_rec!(json!(1), None), vec![(index!(), json!(1))]);
        assert_eq!(
            iter_rec!(json!("string"), None),
            vec![(index!(), json!("string"))]
        );
        assert_eq!(iter_rec!(json!(true), None), vec![(index!(), json!(true))]);
        assert_eq!(iter_rec!(json!(null), None), vec![(index!(), json!(null))]);
    }

    #[test]
    fn non_terminal_value_array_iter_recursive_dfs_limit() {
        let value = json!([
            { "nested": [1, 2, 3] },
            1,
            2,
            3,
        ]);
        let dfs = Iter::new(&value);
        let expected = vec![
            (index!(), value.clone()),
            (index!(0), json!({ "nested": [1, 2, 3] })),
            (index!(0, "nested"), json!([1, 2, 3])),
            (index!(0, "nested", 0), json!(1)),
            (index!(0, "nested", 1), json!(2)),
            (index!(0, "nested", 2), json!(3)),
            (index!(1), json!(1)),
            (index!(2), json!(2)),
            (index!(3), json!(3)),
        ];

        assert_eq!(&dfs.clone().limit(None).collect_cloned(), &expected);
        assert_eq!(&dfs.clone().limit(1).collect_cloned(), &expected[..1]);
        assert_eq!(&dfs.clone().limit(3).collect_cloned(), &expected[..3]);
        assert_eq!(&dfs.clone().limit(5).collect_cloned(), &expected[..5]);
        assert_eq!(&dfs.clone().limit(9).collect_cloned(), &expected[..9]);
    }

    #[test]
    fn non_terminal_value_array_iter_recursive_dfs_order() {
        let value = json!([
            1,
            2,
            { "nested": [1, 2, 3] },
        ]);
        // depth 0
        assert_eq!(&iter_rec!(&value, 0), &vec![(index!(), value.clone())]);
        // depth 1
        assert_eq!(
            &iter_rec!(&value, 1),
            &vec![
                (index!(), value.clone()),
                (index!(0), json!(1)),
                (index!(1), json!(2)),
                (index!(2), json!({ "nested": [1, 2, 3] })),
            ]
        );
        // depth 2
        assert_eq!(
            &iter_rec!(&value, 2),
            &vec![
                (index!(), value.clone()),
                (index!(0), json!(1)),
                (index!(1), json!(2)),
                (index!(2), json!({ "nested": [1, 2, 3] })),
                (index!(2, "nested"), json!([1, 2, 3])),
            ]
        );
        // depth 3
        assert_eq!(
            &iter_rec!(&value, 3),
            &vec![
                (index!(), value),
                (index!(0), json!(1)),
                (index!(1), json!(2)),
                (index!(2), json!({ "nested": [1, 2, 3] })),
                (index!(2, "nested"), json!([1, 2, 3])),
                (index!(2, "nested", 0), json!(1)),
                (index!(2, "nested", 1), json!(2)),
                (index!(2, "nested", 2), json!(3)),
            ]
        );
    }

    #[test]
    fn nonterminal_value_object_iter_recursive_dfs_order() {
        let value = json!({
            "a": 42,
            "person": {
                "name": "John",
                "surname": "Doe"
            },
            "values": [ true, 10, "string" ]
        });

        // depth 0
        assert_eq!(iter_rec!(&value, 0), vec![(index!(), value.clone())]);
        // depth 1
        assert_eq!(
            iter_rec!(&value, 1),
            vec![
                (index!(), value.clone()),
                (index!("a"), json!(42)),
                (
                    index!("person"),
                    json!({
                        "name": "John",
                        "surname": "Doe"
                    })
                ),
                (index!("values"), json!([true, 10, "string"])),
            ]
        );
        // depth 2
        assert_eq!(
            &iter_rec!(&value, 2),
            &vec![
                (index!(), value.clone()),
                (index!("a"), json!(42)),
                (
                    index!("person"),
                    json!({
                        "name": "John",
                        "surname": "Doe"
                    })
                ),
                (index!("person", "name"), json!("John")),
                (index!("person", "surname"), json!("Doe")),
                (index!("values"), json!([true, 10, "string"])),
                (index!("values", 0), json!(true)),
                (index!("values", 1), json!(10)),
                (index!("values", 2), json!("string")),
            ]
        );
        // dfs completes at depth 2 already
        assert_eq!(&iter_rec!(&value, 2), &iter_rec!(&value, 3));
    }

    #[test]
    fn nonterminal_value_object_iter_mut_recursive_dfs_order() {
        let value = json!({
            "a": 42,
            "person": {
                "name": "john",
            },
            "values": [ true, 10, { "12": 1, "null": null } ]
        });

        let invert_value = |_index: &IndexPath, val: &mut Value| {
            use serde_json::Number as Num;
            match val {
                Value::Array(ref mut arr) => {
                    arr.reverse();
                }
                Value::String(ref mut s) => {
                    *s = s.chars().rev().collect::<String>();
                }
                Value::Bool(ref mut b) => {
                    *b = !*b;
                }
                Value::Number(ref mut n) => {
                    let negated = if n.is_i64() || n.is_u64() {
                        Num::from(-n.as_i64().unwrap())
                    } else if n.is_f64() {
                        Num::from_f64(-n.as_f64().unwrap()).unwrap()
                    } else {
                        unreachable!("json numbers are i64, u64, or f64");
                    };
                    *n = negated;
                }
                Value::Object(_) | Value::Null => {}
            }
        };

        macro_rules! inv_rec {
            ( $value:expr, $depth:expr ) => {{
                let mut tmp = $value;
                let mut dfs_mut = IterMut::new(&mut tmp).depth($depth);
                dfs_mut.for_each(invert_value);
                tmp.clone()
            }};
        }

        // depth 0
        assert_eq!(&inv_rec!(value.clone(), 0), &value);
        // depth 1
        assert_eq!(
            &inv_rec!(value.clone(), 1),
            &json!({
                // negated
                "a": -42,
                "person": {
                    "name": "john",
                },
                // reversed
                "values": [ { "12": 1, "null": null }, 10, true ]
            })
        );
        // depth 2
        assert_eq!(
            &inv_rec!(value.clone(), 2),
            &json!({
                // negated
                "a": -42,
                "person": {
                    // reversed
                    "name": "nhoj",
                },
                // reversed
                "values": [
                    { "12": 1, "null": null },
                    // negated
                    -10,
                    // inverted
                    false
                ]
            })
        );

        // depth 3
        assert_eq!(
            &inv_rec!(value.clone(), 3),
            &json!({
                // negated
                "a": -42,
                "person": {
                    // reversed
                    "name": "nhoj",
                },
                // reversed
                "values": [
                    // negated
                    { "12": -1, "null": null },
                    // negated
                    -10,
                    // inverted
                    false
                ]
            })
        );
        // dfs completes at depth 3 already
        assert_eq!(&inv_rec!(value.clone(), 3), &inv_rec!(value, 4));
    }

    #[test]
    fn nonterminal_value_object_iter_mut_recursive_dfs_remove_entries() {
        let value = json!({
            "nested": {
                "key": "value",
                "remove": "i will be removed",
                "nested": {
                    "change": [ "valid", "remove" ],
                    "remove": { "key": "i will be removed"},
                },
            },
        });
        let remove_entries = |_index: &IndexPath, val: &mut Value| {
            match val {
                Value::Array(ref mut arr) => {
                    // remove items
                    arr.retain(|val| val != &json!("remove"));
                }
                Value::Object(ref mut map) => {
                    map.remove("remove");
                }
                _ => {}
            }
        };

        macro_rules! remove_rec {
            ( $value:expr, $depth:expr ) => {{
                let mut tmp = $value;
                let mut dfs_mut = IterMut::new(&mut tmp).depth($depth);
                dfs_mut.for_each(remove_entries);
                tmp.clone()
            }};
        }
        assert_eq!(
            remove_rec!(value, None),
            json!({
                "nested": {
                    "key": "value",
                    // "remove": "i will be removed",
                    "nested": {
                        "change": [ "valid", /* "remove" */ ],
                        // "remove": { "key": "i will be removed"},
                    },
                },
            })
        );
    }

    #[test]
    fn nonterminal_value_object_iter_mut_recursive_dfs_add_entries() {
        let value = json!({
            "nested": {
                "old": "value",
                "nested": {
                    "change": [ "old" ],
                    "nested": { "old": "old value"},
                },
            },
        });
        let add_entries = |_index: &IndexPath, val: &mut Value| {
            match val {
                Value::Array(ref mut arr) => {
                    // add a new entry
                    arr.push(json!({}));
                }
                Value::Object(ref mut map) => {
                    // add a new entry
                    map.insert("new".into(), json!({}));
                }
                _ => {}
            }
        };

        macro_rules! add_rec {
            ( $value:expr, $depth:expr ) => {{
                let mut tmp = $value;
                let mut dfs_mut = IterMut::new(&mut tmp).depth($depth);
                dfs_mut.for_each(add_entries);
                tmp.clone()
            }};
        }
        assert_eq!(
            // must set depth otherwise keeps adding elements infinitely
            add_rec!(value, 3),
            json!({
                "nested": {
                    "old": "value",
                    "new": {
                        "new": {
                            "new": { },
                        },
                    },
                    "nested": {
                        "change": [ "old", {} ],
                        "nested": { "old": "old value", "new": { } },
                        "new": { "new": { } },
                    },
                },
                "new": {
                    "new": {
                        "new": {
                            "new": { }
                        },
                    },
                },
            })
        );
    }
}
