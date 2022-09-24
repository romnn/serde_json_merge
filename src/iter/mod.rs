pub mod dfs;
use super::{Index, IndexPath, IndexRef};
pub use dfs::Dfs;
use itertools::Itertools;
use serde_json::{Map, Value};
use std::borrow::Borrow;
use std::collections::VecDeque;

pub trait Traverser {
    fn new() -> Self;

    fn set_limit<L>(&mut self, limit: L)
    where
        L: Into<Option<usize>>;

    fn set_depth<D>(&mut self, depth: D)
    where
        D: Into<Option<usize>>;

    fn mutate_then_next<'b>(
        &mut self,
        value: &mut Value,
        mutate: impl Fn(&IndexPath, &mut Value) -> (),
    ) -> Option<IndexPath>;

    fn next(&mut self, value: &Value) -> Option<IndexPath>;

    fn reset(&mut self);
}

#[derive(Clone)]
pub struct KeyValueIter<'a, T> {
    inner: &'a Value,
    traverser: T,
}

impl<'a, T> Iterator for KeyValueIter<'a, T>
where
    T: Traverser,
{
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
    }
}

pub struct KeyValueIterMut<'a, T = Dfs> {
    inner: &'a mut Value,
    traverser: T,
}

impl<'a, T> KeyValueIterMut<'a, T>
where
    T: Traverser,
{
    pub fn for_each(&mut self, func: impl Fn(&IndexPath, &mut Value) -> ()) {
        self.traverser.reset();
        while let Some(_) = self.traverser.mutate_then_next(&mut self.inner, &func) {}
    }
}

pub trait Iter // pub trait Iter<T = Dfs>
// where
//     T: Traverser,
{
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
    // fn iter<'a>(&'a self) -> KeyValueIter<'a, T>;

    // fn iter_mut<'a>(&'a mut self) -> KeyValueIterMut<'a, T>;

    // fn iter_recursive<'a>(&'a self) -> KeyValueIter<'a, T>;

    fn iter<'a, T>(&'a self) -> KeyValueIter<'a, T>
    where
        T: Traverser;

    fn iter_mut<'a, T>(&'a mut self) -> KeyValueIterMut<'a, T>
    where
        T: Traverser;

    fn iter_recursive<'a, T>(&'a self) -> KeyValueIter<'a, T>
    where
        T: Traverser;

    // fn iter_mut<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut
    // where
    //     D: Into<Option<usize>>;
    // fn iter_recursive<'a, D>(&'a self, depth: D) -> KeyValueIter<'a, Dfs>
    // where
    //     D: Into<Option<usize>>;
    // fn iter_mut_recursive<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut
    // where
    //     D: Into<Option<usize>>;

    // fn iter<'a, D>(&'a self, depth: D) -> KeyValueIter<'a, Dfs>
    // where
    //     D: Into<Option<usize>>;
    // fn iter_mut<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut
    // where
    //     D: Into<Option<usize>>;
    // fn iter_recursive<'a, D>(&'a self, depth: D) -> KeyValueIter<'a, Dfs>
    // where
    //     D: Into<Option<usize>>;
    // fn iter_mut_recursive<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut
    // where
    //     D: Into<Option<usize>>;

    // iter recursive (depth)
    // iter recursive dfs (depth)
    // iter recursive bfs (depth)

    // none of them can be mut
}

// impl<T> Iter<T> for serde_json::Value
// where
//     T: Traverser,
impl Iter for serde_json::Value {
    fn iter<'a, T>(&'a self) -> KeyValueIter<'a, T>
    where
        T: Traverser,
    {
        let mut traverser = T::new();
        traverser.set_depth(1);
        traverser.set_limit(None);
        KeyValueIter {
            inner: self,
            traverser,
        }
    }

    fn iter_mut<'a, T>(&'a mut self) -> KeyValueIterMut<'a, T>
    where
        T: Traverser,
    {
        let mut traverser = T::new();
        traverser.set_depth(1);
        traverser.set_limit(None);
        KeyValueIterMut {
            inner: self,
            traverser,
        }
    }

    fn iter_recursive<'a, T>(&'a self) -> KeyValueIter<'a, T>
    where
        T: Traverser,
    {
        let mut traverser = T::new();
        traverser.set_depth(None);
        traverser.set_limit(None);
        KeyValueIter {
            inner: self,
            traverser,
        }
    }

    // fn iter_recursive<'a, T = DfsTraverser>(&'a self, depth: D) -> KeyValueIter<'a, Dfs>
    // where
    //     D: Into<Option<usize>>,
    // {
    //     let dfs = Dfs::with_depth(depth);
    //     KeyValueIter::new(self, dfs)
    // }

    // fn iter_mut_recursive<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut<'a>
    // where
    //     D: Into<Option<usize>>,
    // {
    //     KeyValueIterMut::new(self, depth)
    // }

    // fn iter_recursive<'a, D>(&'a self, depth: D) -> KeyValueIter<'a, Dfs>
    // where
    //     D: Into<Option<usize>>,
    // {
    //     let dfs = Dfs::with_depth(depth);
    //     KeyValueIter::new(self, dfs)
    // }

    // fn iter_mut_recursive<'a, D>(&'a mut self, depth: D) -> KeyValueIterMut<'a>
    // where
    //     D: Into<Option<usize>>,
    // {
    //     KeyValueIterMut::new(self, depth)
    // }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::index;
    use crate::test::CollectCloned;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    #[test]
    fn nonterminal_value_iter() {
        let value = json!({
            "person1": { "name": "bob" },
            "person2": { "name": "john" },
        });
        assert_eq!(
            value.iter::<Dfs>().collect_cloned(),
            vec![
                //  todo: depth 0 should be skipped
                (index!(), value.clone()),
                (index!("person1"), json!({ "name": "bob" })),
                (index!("person2"), json!({ "name": "john" })),
            ]
        );
        // todo: same as bfs
        assert_eq!(
            value.iter::<Dfs>().collect_cloned(),
            value.iter::<Dfs>().collect_cloned()
        );
    }

    #[test]
    fn value_iter_recursive_dfs() {
        let value = json!({
            "person1": { "name": "bob" },
            "person2": { "name": "john" },
        });
        assert_eq!(
            value.iter_recursive::<Dfs>().collect_cloned(),
            vec![
                (index!(), value.clone()),
                (index!("person1"), json!({ "name": "bob" })),
                (index!("person1", "name"), json!("bob")),
                (index!("person2"), json!({ "name": "john" })),
                (index!("person2", "name"), json!("john")),
            ]
        );
    }
}
