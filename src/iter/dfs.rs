use super::{KeyValueIter, Traverser};
use crate::{Index, IndexPath, IndexRef};
use itertools::Itertools;
use serde_json::{Map, Value};
use std::borrow::Borrow;
use std::collections::VecDeque;

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
            queue: VecDeque::from_iter([(0, IndexPath::new())]),
            depth: None,
            limit: None,
            num_visited: 0,
        }
    }
}

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
        self.num_visited = 0;
    }

    #[inline]
    fn mutate_then_next<'b>(
        &mut self,
        value: &mut Value,
        mutate: impl Fn(&IndexPath, &mut Value) -> (),
    ) -> Option<IndexPath> {
        match self.queue.pop_back() {
            Some((depth, index)) => {
                // mutate before adding children to queue
                if let Some(mut val) = value.get_index_mut(&index) {
                    mutate(&index, val);
                }
                if self.depth.map(|d| depth < d).unwrap_or(true) {
                    // add children
                    match value.get_index(&index) {
                        Some(Value::Object(o)) => {
                            self.queue.extend(o.keys().map(|key| {
                                let mut index = index.clone();
                                index.push(key.clone());
                                (depth + 1, index)
                            }));
                        }
                        Some(Value::Array(arr)) => {
                            self.queue
                                .extend(arr.iter().enumerate().rev().map(|(arr_idx, _)| {
                                    let mut index = index.clone();
                                    index.push(arr_idx.clone());
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
        match self.queue.pop_back() {
            Some((depth, index)) => {
                if self.depth.map(|d| depth < d).unwrap_or(true) {
                    // add children
                    match value.get_index(&index) {
                        Some(Value::Object(map)) => {
                            self.queue.extend(map.keys().rev().map(|key| {
                                let mut index = index.clone();
                                index.push(key.clone());
                                (depth + 1, index)
                            }));
                        }
                        Some(Value::Array(arr)) => {
                            self.queue
                                .extend(arr.iter().enumerate().rev().map(|(arr_idx, _)| {
                                    let mut index = index.clone();
                                    index.push(arr_idx.clone());
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
}

pub struct DfsIter<'a> {
    inner: &'a Value,
    limit: Option<usize>,
    depth: Option<usize>,
}

impl<'a> DfsIter<'a> {
    #[inline]
    pub fn new(value: &'a Value) -> Self {
        Self {
            inner: value,
            depth: None,
            limit: None,
        }
    }

    #[inline]
    pub fn depth(self, depth: impl Into<Option<usize>>) -> Self {
        Self {
            depth: depth.into(),
            ..self
        }
    }

    #[inline]
    pub fn limit(self, limit: impl Into<Option<usize>>) -> Self {
        Self {
            limit: limit.into(),
            ..self
        }
    }
}

impl<'a> IntoIterator for DfsIter<'a> {
    type Item = <KeyValueIter<'a, Dfs> as Iterator>::Item;
    type IntoIter = KeyValueIter<'a, Dfs>;

    fn into_iter(self) -> Self::IntoIter {
        let traverser = Dfs {
            depth: self.depth,
            limit: self.limit,
            ..Default::default()
        };
        KeyValueIter {
            inner: self.inner,
            traverser,
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::index;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    #[test]
    fn test_object_iter_recursive_dfs() {
        let mut value = json!({
            "a": 42,
            "person": {
                "name": "John",
                "surname": "Doe"
            },
            "values": [ true, 10, "string" ]
        });
        macro_rules! iter_rec {
            ( $value:expr, $depth:expr ) => {{
                let dfs = DfsIter::new($value).depth($depth);
                dfs.into_iter()
                    .map(|(index, value)| (index, value.clone()))
                    .collect::<Vec<(IndexPath, Value)>>()
            }};
        }

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
            iter_rec!(&value, 2),
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
                (index!("person", "name"), json!("John")),
                (index!("person", "surname"), json!("Doe")),
                (index!("values"), json!([true, 10, "string"])),
                (index!("values", 0), json!(true)),
                (index!("values", 1), json!(10)),
                (index!("values", 2), json!("string")),
            ]
        );
        // dfs is finished at depth 2 already
        assert_eq!(iter_rec!(&value, 2), iter_rec!(&value, 3))
    }
}
