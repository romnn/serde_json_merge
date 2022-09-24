//use super::{Index, IndexPath, IndexRef};
//use itertools::Itertools;
//use serde_json::{Map, Value};
//use std::borrow::Borrow;
//use std::collections::VecDeque;

//pub trait Traverser {
//    fn new() -> Self;

//    fn set_limit<L>(&mut self, limit: L)
//    where
//        L: Into<Option<usize>>;

//    fn set_depth<D>(&mut self, depth: D)
//    where
//        D: Into<Option<usize>>;

//    fn mutate_then_next<'b>(
//        &mut self,
//        value: &mut Value,
//        mutate: impl Fn(&IndexPath, &mut Value) -> (),
//    ) -> Option<IndexPath>;

//    fn next(&mut self, value: &Value) -> Option<IndexPath>;

//    fn reset(&mut self);
//}

//pub struct Dfs {
//    queue: VecDeque<(usize, IndexPath)>,
//    depth: Option<usize>,
//    limit: Option<usize>,
//    num_visited: usize,
//}

//impl Default for Dfs {
//    #[inline]
//    fn default() -> Self {
//        Self {
//            queue: VecDeque::from_iter([(0, IndexPath::new())]),
//            depth: None,
//            limit: None,
//            num_visited: 0,
//        }
//    }
//}

//impl Traverser for Dfs {
//    #[inline]
//    fn new() -> Self {
//        Self::default()
//    }

//    #[inline]
//    fn set_limit<L>(&mut self, limit: L)
//    where
//        L: Into<Option<usize>>,
//    {
//        self.limit = limit.into();
//    }

//    #[inline]
//    fn set_depth<D>(&mut self, depth: D)
//    where
//        D: Into<Option<usize>>,
//    {
//        self.depth = depth.into();
//    }

//    #[inline]
//    fn reset(&mut self) {
//        self.queue.clear();
//        self.num_visited = 0;
//    }

//    #[inline]
//    fn mutate_then_next<'b>(
//        &mut self,
//        value: &mut Value,
//        mutate: impl Fn(&IndexPath, &mut Value) -> (),
//    ) -> Option<IndexPath> {
//        match self.queue.pop_back() {
//            Some((depth, index)) => {
//                // mutate before adding children to queue
//                if let Some(mut val) = value.get_index_mut(&index) {
//                    mutate(&index, val);
//                }
//                if self.depth.map(|d| depth < d).unwrap_or(true) {
//                    // add children
//                    match value.get_index(&index) {
//                        Some(Value::Object(o)) => {
//                            self.queue.extend(o.keys().map(|key| {
//                                let mut index = index.clone();
//                                index.push(key.clone());
//                                (depth + 1, index)
//                            }));
//                        }
//                        Some(Value::Array(arr)) => {
//                            self.queue
//                                .extend(arr.iter().enumerate().rev().map(|(arr_idx, _)| {
//                                    let mut index = index.clone();
//                                    index.push(arr_idx.clone());
//                                    (depth + 1, index)
//                                }));
//                        }
//                        _ => {}
//                    }
//                }
//                Some(index)
//            }
//            None => None,
//        }
//    }

//    #[inline]
//    fn next(&mut self, value: &Value) -> Option<IndexPath> {
//        match self.queue.pop_back() {
//            Some((depth, index)) => {
//                if self.depth.map(|d| depth < d).unwrap_or(true) {
//                    // add children
//                    match value.get_index(&index) {
//                        Some(Value::Object(map)) => {
//                            self.queue.extend(map.keys().rev().map(|key| {
//                                let mut index = index.clone();
//                                index.push(key.clone());
//                                (depth + 1, index)
//                            }));
//                        }
//                        Some(Value::Array(arr)) => {
//                            self.queue
//                                .extend(arr.iter().enumerate().rev().map(|(arr_idx, _)| {
//                                    let mut index = index.clone();
//                                    index.push(arr_idx.clone());
//                                    (depth + 1, index)
//                                }));
//                        }
//                        _ => {}
//                    }
//                }
//                Some(index)
//            }
//            None => None,
//        }
//    }
//}

//pub struct KeyValueIterMut<'a, T = Dfs> {
//    inner: &'a mut Value,
//    traverser: T,
//    // depth: Option<usize>,
//}

//impl<'a, T> KeyValueIterMut<'a, T> {
//    // pub fn new(inner: &'a mut Value, depth: impl Into<Option<usize>>) -> Self {
//    pub fn new(inner: &'a mut Value, traverser: T) -> Self {
//        // pub fn new(inner: &'a mut Value) -> Self {
//        Self {
//            inner,
//            traverser,
//            // depth: depth.into(),
//        }
//    }
//}

//impl<'a, T> KeyValueIterMut<'a, T>
//where
//    T: Traverser,
//{
//    pub fn for_each(&mut self, func: impl Fn(&IndexPath, &mut Value) -> ()) {
//        // let mut dfs = Dfs::with_depth(None); //  self.depth);
//        // let mut traverser = T::with_depth(self.depth);
//        self.traverser.reset();
//        while let Some(_) = self.traverser.mutate_then_next(&mut self.inner, &func) {}
//        // loop {
//        //     match self.traverser.mutate_then_next(&mut self.inner, &func)
//        // }
//        // self.traverser
//        //     .mutate_then_next(&mut self.inner, &func)
//        //     .take_until(|val| val.is_none())
//    }
//}

//pub struct DfsIter<'a> {
//    inner: &'a Value,
//    limit: Option<usize>,
//    depth: Option<usize>,
//}

//// let dfs = DfsIter::new(&value).depth(10).limit(10);
//// for item in dfs {}

//// let dfs = DfsIter::new(&value).depth(10).limit(10);
//// for item in dfs.into_iter() {}

//// let dfs = DfsIter::new().depth(10).limit(10);
//// for item in dfs.iter(&value) {}

//impl<'a> DfsIter<'a> {
//    #[inline]
//    pub fn new(value: &'a Value) -> Self {
//        Self {
//            inner: value,
//            depth: None,
//            limit: None,
//        }
//    }

//    #[inline]
//    pub fn depth(self, depth: impl Into<Option<usize>>) -> Self {
//        Self {
//            depth: depth.into(),
//            ..self
//        }
//    }

//    #[inline]
//    pub fn limit(self, limit: impl Into<Option<usize>>) -> Self {
//        Self {
//            limit: limit.into(),
//            ..self
//        }
//    }
//}

//impl<'a> IntoIterator for DfsIter<'a> {
//    type Item = <KeyValueIter<'a, Dfs> as Iterator>::Item;
//    type IntoIter = KeyValueIter<'a, Dfs>;

//    fn into_iter(self) -> Self::IntoIter {
//        let traverser = Dfs {
//            depth: self.depth,
//            limit: self.limit,
//            ..Default::default()
//        };
//        KeyValueIter {
//            inner: self.inner,
//            traverser,
//        }
//    }
//}

//pub struct KeyValueIter<'a, T> {
//    inner: &'a Value,
//    traverser: T,
//}

//// impl<'a, T> KeyValueIter<'a, T> {
////     pub fn new(inner: &'a Value, traverser: T) -> Self {
////         Self { inner, traverser }
////     }
//// }

//impl<'a, T> Iterator for KeyValueIter<'a, T>
//where
//    T: Traverser,
//{
//    type Item = (IndexPath, &'a Value);

//    #[inline]
//    fn next(&mut self) -> Option<Self::Item> {
//        loop {
//            match self.traverser.next(&self.inner).map(|idx| {
//                let value = self.inner.get_index(&idx);
//                (idx, value)
//            }) {
//                Some((idx, Some(value))) => return Some((idx, value)),
//                Some(_) => continue,
//                None => return None,
//            }
//        }
//    }
//}

//pub trait Iter {
//    // fn keys<'a>(&'a self) -> IndexIter<'a>;
//    // keys
//    // keys recursive (depth)
//    // keys recursive bfs (depth)
//    // keys recursive dfs (depth)
//    // values
//    // values recursive (depth)
//    // values recursive dfs (depth)
//    // values recursive bfs (depth)
//    //
//    // iter (depth)
//    fn iter<'a, T>(&'a self) -> KeyValueIter<'a, T>
//    where
//        T: Traverser;
//    fn iter_recursive<'a, T>(&'a self) -> KeyValueIter<'a, T>
//    where
//        T: Traverser;

//    // fn iter_mut<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut
//    // where
//    //     D: Into<Option<usize>>;
//    // fn iter_recursive<'a, D>(&'a self, depth: D) -> KeyValueIter<'a, Dfs>
//    // where
//    //     D: Into<Option<usize>>;
//    // fn iter_mut_recursive<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut
//    // where
//    //     D: Into<Option<usize>>;

//    // fn iter<'a, D>(&'a self, depth: D) -> KeyValueIter<'a, Dfs>
//    // where
//    //     D: Into<Option<usize>>;
//    // fn iter_mut<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut
//    // where
//    //     D: Into<Option<usize>>;
//    // fn iter_recursive<'a, D>(&'a self, depth: D) -> KeyValueIter<'a, Dfs>
//    // where
//    //     D: Into<Option<usize>>;
//    // fn iter_mut_recursive<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut
//    // where
//    //     D: Into<Option<usize>>;

//    // iter recursive (depth)
//    // iter recursive dfs (depth)
//    // iter recursive bfs (depth)

//    // none of them can be mut
//}

//impl Iter for serde_json::Value {
//    fn iter<'a, T>(&'a self) -> KeyValueIter<'a, T>
//    where
//        T: Traverser,
//    {
//        let mut traverser = T::new();
//        traverser.set_depth(1);
//        traverser.set_limit(None);
//        // KeyValueIter::new(self, traverser)
//        KeyValueIter {
//            inner: self,
//            traverser,
//        }
//    }

//    fn iter_recursive<'a, T>(&'a self) -> KeyValueIter<'a, T>
//    where
//        T: Traverser,
//    {
//        let mut traverser = T::new();
//        traverser.set_depth(None);
//        traverser.set_limit(None);
//        KeyValueIter {
//            inner: self,
//            traverser,
//        }
//    }

//    // fn iter_recursive<'a, T = DfsTraverser>(&'a self, depth: D) -> KeyValueIter<'a, Dfs>
//    // where
//    //     D: Into<Option<usize>>,
//    // {
//    //     let dfs = Dfs::with_depth(depth);
//    //     KeyValueIter::new(self, dfs)
//    // }

//    // fn iter_mut_recursive<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut<'a>
//    // where
//    //     D: Into<Option<usize>>,
//    // {
//    //     KeyValueIterMut::new(self, depth)
//    // }

//    // fn iter_recursive<'a, D>(&'a self, depth: D) -> KeyValueIter<'a, Dfs>
//    // where
//    //     D: Into<Option<usize>>,
//    // {
//    //     let dfs = Dfs::with_depth(depth);
//    //     KeyValueIter::new(self, dfs)
//    // }

//    // fn iter_mut_recursive<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut<'a>
//    // where
//    //     D: Into<Option<usize>>,
//    // {
//    //     KeyValueIterMut::new(self, depth)
//    // }
//}

//#[cfg(test)]
//pub mod test {
//    use super::*;
//    use crate::index;
//    use anyhow::Result;
//    use pretty_assertions::assert_eq;
//    use serde_json::{json, Value};

//    #[test]
//    fn test_hash_map_iter() {
//        let map = std::collections::HashMap::<String, String>::new();
//        // good to know this does not work
//        // let test = map.iter().cloned();
//    }

//    // todo: test the iter and iter mut dfs variants now
//    // todo: use dfs for the normal iter
//    #[test]
//    fn test_object_keys_recursively_dfs() {
//        // macro_rules! iter_keys {
//        //     ( $value:ident, $depth:expr ) => {{
//        //         $value
//        //             .iter_recursive($depth)
//        //             .map(|(index, value)| index)
//        //             .collect::<Vec<IndexPath>>()
//        //     }};
//        // }

//        // assert_eq!(iter_keys!(value, 0), vec![index!()]);
//        // assert_eq!(
//        //     iter_keys!(value, 1),
//        //     vec![index!(), index!("values"), index!("person"), index!("a")]
//        // );
//    }

//    #[test]
//    fn test_object_iter_recursive_dfs() {
//        let mut value = json!({
//            "a": 42,
//            "person": {
//                "name": "John",
//                "surname": "Doe"
//            },
//            "values": [ true, 10, "string" ]
//        });
//        macro_rules! iter_rec {
//            ( $value:expr, $depth:expr ) => {{
//                let dfs = DfsIter::new($value).depth($depth);
//                dfs.into_iter()
//                    .map(|(index, value)| (index, value.clone()))
//                    .collect::<Vec<(IndexPath, Value)>>()
//            }};
//        }

//        // depth 0
//        assert_eq!(iter_rec!(&value, 0), vec![(index!(), value.clone())]);
//        // depth 1
//        assert_eq!(
//            iter_rec!(&value, 1),
//            vec![
//                (index!(), value.clone()),
//                (index!("a"), json!(42)),
//                (
//                    index!("person"),
//                    json!({
//                        "name": "John",
//                        "surname": "Doe"
//                    })
//                ),
//                (index!("values"), json!([true, 10, "string"])),
//            ]
//        );
//        // depth 2
//        assert_eq!(
//            iter_rec!(&value, 2),
//            vec![
//                (index!(), value.clone()),
//                (index!("a"), json!(42)),
//                (
//                    index!("person"),
//                    json!({
//                        "name": "John",
//                        "surname": "Doe"
//                    })
//                ),
//                (index!("person", "name"), json!("John")),
//                (index!("person", "surname"), json!("Doe")),
//                (index!("values"), json!([true, 10, "string"])),
//                (index!("values", 0), json!(true)),
//                (index!("values", 1), json!(10)),
//                (index!("values", 2), json!("string")),
//            ]
//        );
//        // dfs is finished at depth 2 already
//        assert_eq!(iter_rec!(&value, 2), iter_rec!(&value, 3))
//    }

//    #[test]
//    fn test_iter_keys_recursively() {
//        let mut value = json!({
//            "title": "This is a title",
//            "person" : {
//                "firstName" : "John",
//                "lastName" : "Doe"
//            },
//            "cities": [ "london", "paris" ]
//        });
//        // .iter_mut().for_each(|i| *i *= 2)
//        // for item in value.iter_recursive(None).map(|item| item.clone()) {

//        //// let iter = value.iter_mut_recursive(None);
//        //let mut dfs = Dfs::new();
//        //while let Some(idx) = dfs.next(&value) {
//        //    // mutate in here
//        //    let mut test = value.get_index_mut(idx);
//        //}

//        //let mut iter = value.iter_mut_recursive();
//        //iter.for_each(|index: &IndexPath, val: &mut Value| {
//        //    // *val = json!("test");
//        //    dbg!(index, val);
//        //});

//        //// for item in value.iter_mut_recursive(None) {}
//        ////

//        //for item in value.clone().iter_recursive() {
//        //    let (idx, val): (IndexPath, &Value) = item;
//        //    // get mutable access
//        //    let mut test = value.get_index_mut(idx);
//        //    assert_eq!(Some(val.clone()), test.cloned());
//        //    dbg!(&val);
//        //}
//        //// assert!(false);
//    }
//}
