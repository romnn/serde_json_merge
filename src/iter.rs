use super::{Index, IndexPath, IndexRef};
use itertools::Itertools;
use serde_json::{Map, Value};
use std::collections::VecDeque;

pub struct Dfs<'a>
// pub struct Dfs<CB>
// where
//     CB: Fn(&IndexPath),
{
    queue: VecDeque<(usize, IndexPath)>,
    depth: Option<usize>,
    mutate: Option<Box<dyn FnMut(&IndexPath) -> () + 'a>>,
    // mutate: Option<CB>,
    // mutate: Option<&dyn Fn(IndexPath) -> ()>,
}

impl<'a> Default for Dfs<'a>
// impl<CB> Default for Dfs<CB>
// where
//     CB: Fn(&IndexPath),
{
    fn default() -> Self {
        Self {
            queue: VecDeque::from_iter([(0, IndexPath::new())]),
            depth: None,
            mutate: None,
        }
    }
}

// impl<'cb, CB> Dfs<'cb, CB>
impl<'a> Dfs<'a>
// impl<CB> Dfs<CB>
// where
//     CB: Fn(&IndexPath), //  -> () + 'cb,
{
    pub fn new() -> Self {
        Self::with_depth(None)
    }

    pub fn builder() -> Self {
        Self::default()
    }

    pub fn callback(self, cb: impl FnMut(&IndexPath) -> () + 'a) -> Self {
        Self {
            mutate: Some(Box::new(cb)),
            ..self
        }
    }

    pub fn depth(self, depth: impl Into<Option<usize>>) -> Self {
        Self {
            depth: depth.into(),
            ..self
        }
    }

    pub fn with_depth(depth: impl Into<Option<usize>>) -> Self {
        Self {
            // queue: VecDeque::from_iter([(0, IndexPath::new())]),
            depth: depth.into(),
            // mutate: None,
            ..Default::default()
        }
    }
}

// impl<'cb, CB> Dfs<'cb, CB>
impl<'a> Dfs<'a>
// impl<CB> Dfs<CB>
// where
//     CB: Fn(&IndexPath),
{
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
                Some(index)
            }
            None => None,
        }
    }

    #[inline]
    fn next(&mut self, value: &Value) -> Option<IndexPath> {
        // self.mutate_then_next(value, |_| {})
        match self.queue.pop_back() {
            Some((depth, index)) => {
                // callback before adding children
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
                Some(index)
            }
            None => None,
        }
    }
}

pub struct KeyValueIterMut<'a> {
    inner: &'a mut Value,
    // queue: VecDeque<(usize, IndexPath)>,
    depth: Option<usize>,
}

impl<'a> KeyValueIterMut<'a> {
    pub fn new(inner: &'a mut Value, depth: impl Into<Option<usize>>) -> Self {
        Self {
            inner,
            // queue: VecDeque::from_iter([(0, IndexPath::new())]),
            depth: depth.into(),
        }
    }
}

impl<'a> KeyValueIterMut<'a> {
    pub fn for_each(&mut self, func: impl Fn(&IndexPath, &mut Value) -> ()) {
        // let mut queue = VecDeque::from_iter([(0, IndexPath::new())]);
        // while let Some((depth, index)) = queue.pop_back() {
        //     // mutate before adding children to queue
        //     if let Some(mut val) = self.inner.get_index_mut(&index) {
        //         func(&index, val);
        //     }
        //     // add children
        //     match self.inner.get_index(&index) {
        //         Some(Value::Object(o)) => {
        //             queue.extend(o.keys().map(|key| {
        //                 let mut index = index.clone();
        //                 index.push(key.clone());
        //                 (depth + 1, index)
        //             }));
        //         }
        //         _ => {}
        //     }
        // }
        // // self.queue.clear();
        let mut dfs = Dfs::with_depth(self.depth);
        // .callback(|index: &IndexPath| {
        // let mut value = self.inner.get_index_mut(index);
        // func(
        // todo: mutate index here
        // });
        // dfs.collect();
        // while let Some(idx) = dfs.next(&self.inner) {
        while let Some(idx) = dfs.mutate_then_next(&mut self.inner, &func) {
            // mutate in here
            // let mut test = value.get_index_mut(idx);
        }

        // todo!();
    }
}

pub struct KeyValueIter<'a> {
    inner: &'a Value,
    queue: VecDeque<(usize, IndexPath)>,
    depth: Option<usize>,
}

impl<'a> KeyValueIter<'a> {
    pub fn new(inner: &'a Value, depth: impl Into<Option<usize>>) -> Self {
        Self {
            inner,
            queue: VecDeque::from_iter([(0, IndexPath::new())]),
            depth: depth.into(),
        }
    }
}

impl<'a> Iterator for KeyValueIter<'a> {
    type Item = (IndexPath, &'a Value);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.queue.pop_back() {
                Some((depth, index)) => {
                    match self.inner.get_index(&index) {
                        Some(Value::Object(o)) => {
                            self.queue.extend(o.keys().map(|key| {
                                let mut index = index.clone();
                                index.push(key.clone());
                                (depth + 1, index)
                            }));
                        }
                        _ => {}
                    }
                    if let Some(value) = self.inner.get_index(&index) {
                        return Some((index, value));
                    }
                }
                None => return None,
            }
        }
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
    fn iter_recursive<'a, D>(&'a self, depth: D) -> KeyValueIter<'a>
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
    fn iter_recursive<'a, D>(&'a self, depth: D) -> KeyValueIter<'a>
    where
        D: Into<Option<usize>>,
    {
        KeyValueIter::new(self, depth)
    }

    // fn iter_mut_recursive<'a, D>(&'a mut self, depth: D) -> IndexIterMut<'a>
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
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    #[test]
    fn test_hash_map_iter() {
        let map = std::collections::HashMap::<String, String>::new();
        // good to know this does not work
        // let test = map.iter().cloned();
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
            dbg!(val);
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
        assert!(false);
    }
}
