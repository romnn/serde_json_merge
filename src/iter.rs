use super::{Index, IndexPath, IndexRef};
use itertools::Itertools;
use serde_json::{Map, Value};
use std::collections::VecDeque;

pub trait Traverser {
    // #[inline]
    // pub fn new() -> Self {
    //     Self::with_depth(None)
    // }

    // #[inline]
    // pub fn with_depth(depth: impl Into<Option<usize>>) -> Self {
    //     Self {
    //         depth: depth.into(),
    //         ..Default::default()
    //     }
    // }

    fn mutate_then_next<'b>(
        &mut self,
        value: &mut Value,
        mutate: impl Fn(&IndexPath, &mut Value) -> (),
    ) -> Option<IndexPath>;

    fn next(&mut self, value: &Value) -> Option<IndexPath>;
}

pub struct Dfs {
    queue: VecDeque<(usize, IndexPath)>,
    depth: Option<usize>,
}

impl Default for Dfs {
    #[inline]
    fn default() -> Self {
        Self {
            queue: VecDeque::from_iter([(0, IndexPath::new())]),
            depth: None,
        }
    }
}

impl Dfs {
    #[inline]
    pub fn new() -> Self {
        Self::with_depth(None)
    }

    #[inline]
    pub fn with_depth(depth: impl Into<Option<usize>>) -> Self {
        Self {
            depth: depth.into(),
            ..Default::default()
        }
    }
}

impl Traverser for Dfs {
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

pub struct KeyValueIterMut<'a> {
    inner: &'a mut Value,
    depth: Option<usize>,
}

impl<'a> KeyValueIterMut<'a> {
    pub fn new(inner: &'a mut Value, depth: impl Into<Option<usize>>) -> Self {
        Self {
            inner,
            depth: depth.into(),
        }
    }
}

impl<'a> KeyValueIterMut<'a> {
    pub fn for_each(&mut self, func: impl Fn(&IndexPath, &mut Value) -> ()) {
        let mut dfs = Dfs::with_depth(self.depth);
        while let Some(idx) = dfs.mutate_then_next(&mut self.inner, &func) {}
    }
}

pub struct KeyValueIter<'a, T> {
    inner: &'a Value,
    traverser: T,
    // queue: VecDeque<(usize, IndexPath)>,
    // depth: Option<usize>,
}

impl<'a, T> KeyValueIter<'a, T> {
    // pub fn new(inner: &'a Value, depth: impl Into<Option<usize>>) -> Self {
    pub fn new(inner: &'a Value, traverser: T) -> Self {
        Self {
            inner,
            traverser,
            // queue: VecDeque::from_iter([(0, IndexPath::new())]),
            // depth: depth.into(),
        }
    }
}

impl<'a, T> Iterator for KeyValueIter<'a, T>
where
    T: Traverser,
{
    // type Item = (IndexPath, &'a Value);
    type Item = (IndexPath, &'a Value);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.traverser.next(&self.inner).map(|idx| {
                let value = self.inner.get_index(&idx);
                (idx, value)
            }) {
                Some((idx, Some(value))) => return Some((idx, value)),
                Some(_) => continue,
                None => return None,
            }
        }
        // while let Some(idx) =  {
        //     // mutate in here
        //     let mut test = value.get_index_mut(idx);
        // }
        // loop {
        //     match self.queue.pop_back() {
        //         Some((depth, index)) => {
        //             match self.inner.get_index(&index) {
        //                 Some(Value::Object(o)) => {
        //                     self.queue.extend(o.keys().map(|key| {
        //                         let mut index = index.clone();
        //                         index.push(key.clone());
        //                         (depth + 1, index)
        //                     }));
        //                 }
        //                 _ => {}
        //             }
        //             if let Some(value) = self.inner.get_index(&index) {
        //                 return Some((index, value));
        //             }
        //         }
        //         None => return None,
        //     }
        // }
    }
}

pub trait Iter {
    // fn keys<'a>(&'a self) -> IndexIter<'a>;
    // keys
    // keys recursive (depth)
    // keys recursive bfs (depth)
    // keys recursive dfs (depth)
    // values
    // values recursive (depth)
    // values recursive dfs (depth)
    // values recursive bfs (depth)
    //
    // iter (depth)
    fn iter_recursive<'a, D>(&'a self, depth: D) -> KeyValueIter<'a, Dfs>
    where
        D: Into<Option<usize>>;
    fn iter_mut_recursive<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut
    where
        D: Into<Option<usize>>;

    // iter recursive (depth)
    // iter recursive dfs (depth)
    // iter recursive bfs (depth)

    // none of them can be mut
}

impl Iter for serde_json::Value {
    fn iter_recursive<'a, D>(&'a self, depth: D) -> KeyValueIter<'a, Dfs>
    where
        D: Into<Option<usize>>,
    {
        let dfs = Dfs::with_depth(depth);
        KeyValueIter::new(self, dfs)
    }

    fn iter_mut_recursive<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut<'a>
    where
        D: Into<Option<usize>>,
    {
        KeyValueIterMut::new(self, depth)
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
    fn test_hash_map_iter() {
        let map = std::collections::HashMap::<String, String>::new();
        // good to know this does not work
        // let test = map.iter().cloned();
    }

    // todo: test the iter and iter mut dfs variants now
    // todo: use dfs for the normal iter
    #[test]
    fn test_object_keys_recursively_dfs() {
        // macro_rules! iter_keys {
        //     ( $value:ident, $depth:expr ) => {{
        //         $value
        //             .iter_recursive($depth)
        //             .map(|(index, value)| index)
        //             .collect::<Vec<IndexPath>>()
        //     }};
        // }

        // assert_eq!(iter_keys!(value, 0), vec![index!()]);
        // assert_eq!(
        //     iter_keys!(value, 1),
        //     vec![index!(), index!("values"), index!("person"), index!("a")]
        // );
    }

    #[test]
    fn test_object_iter_recursively_dfs() {
        let mut value = json!({
            "a": 42,
            "person": {
                "name": "John",
                "surname": "Doe"
            },
            "values": [ true, 10, "string" ]
        });
        macro_rules! iter_rec {
            ( $value:ident, $depth:expr ) => {{
                $value
                    .iter_recursive($depth)
                    .map(|(index, value)| (index, value.clone()))
                    .collect::<Vec<(IndexPath, Value)>>()
            }};
        }

        assert_eq!(iter_rec!(value, 0), vec![(index!(), value.clone())]);
        assert_eq!(
            iter_rec!(value, 1),
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
        assert_eq!(
            iter_rec!(value, 2),
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
    }

    #[test]
    fn test_iter_keys_recursively() {
        let mut value = json!({
            "title": "This is a title",
            "person" : {
                "firstName" : "John",
                "lastName" : "Doe"
            },
            "cities": [ "london", "paris" ]
        });
        // .iter_mut().for_each(|i| *i *= 2)
        // for item in value.iter_recursive(None).map(|item| item.clone()) {

        // let iter = value.iter_mut_recursive(None);
        let mut dfs = Dfs::new();
        while let Some(idx) = dfs.next(&value) {
            // mutate in here
            let mut test = value.get_index_mut(idx);
        }

        let mut iter = value.iter_mut_recursive(None);
        iter.for_each(|index: &IndexPath, val: &mut Value| {
            // *val = json!("test");
            dbg!(index, val);
        });

        // for item in value.iter_mut_recursive(None) {}
        //

        for item in value.clone().iter_recursive(None) {
            let (idx, val): (IndexPath, &Value) = item;
            // get mutable access
            let mut test = value.get_index_mut(idx);
            assert_eq!(Some(val.clone()), test.cloned());
            dbg!(&val);
        }
        // assert!(false);
    }
}
