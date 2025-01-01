pub mod dfs;
use super::{Index, IndexPath};
use serde_json::Value;

#[cfg(feature = "rayon")]
pub trait ParallelTraverser: Sized {
    fn split(&mut self) -> Option<Self>;
}

pub trait Traverser {
    fn new() -> Self;

    fn set_limit<L>(&mut self, limit: L)
    where
        L: Into<Option<usize>>;

    fn set_depth<D>(&mut self, depth: D)
    where
        D: Into<Option<usize>>;

    fn mutate_then_next(
        &mut self,
        value: &mut Value,
        mutate: impl FnMut(&IndexPath, &mut Value),
    ) -> Option<IndexPath>;

    fn next(&mut self, value: &Value) -> Option<IndexPath>;

    fn process_next(
        &mut self,
        value: &Value,
        process: impl FnMut(&IndexPath, Option<&Value>) -> bool,
    ) -> Option<IndexPath>;

    fn reset(&mut self);
}

#[allow(clippy::module_name_repetitions)]
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
            match self.traverser.next(self.inner).map(|idx| {
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

// #[cfg(feature = "rayon")]
// impl<'a, T> par_dfs::sync::par::SplittableIterator for KeyValueIter<'a, T>
// where
//     T: Traverser + ParallelTraverser,
// {
//     fn split(&mut self) -> Option<Self> {
//         match self.traverser.split() {
//             Some(split) => Some(Self {
//                 traverser: split,
//                 inner: self.inner,
//             }),
//             None => None,
//         }
//     }
// }

// #[cfg(feature = "rayon")]
// impl<'a, T> rayon::iter::IntoParallelIterator for KeyValueIter<'a, T>
// where
//     T: Traverser + ParallelTraverser + Send,
// {
//     type Iter = par_dfs::sync::par::ParallelSplittableIterator<Self>;
//     type Item = <Self as Iterator>::Item;

//     fn into_par_iter(self) -> Self::Iter {
//         par_dfs::sync::par::ParallelSplittableIterator::new(self)
//     }
// }

pub struct KeyValueMutator<'a, T> {
    inner: &'a mut Value,
    traverser: T,
}

impl<T> KeyValueMutator<'_, T>
where
    T: Traverser,
{
    pub fn for_each(&mut self, mut func: impl FnMut(&IndexPath, &mut Value)) {
        self.traverser.reset();
        while self
            .traverser
            .mutate_then_next(self.inner, &mut func)
            .is_some()
        {}
    }
}

pub trait Iter {
    fn iter<T>(&self) -> KeyValueIter<T>
    where
        T: Traverser;

    fn mutate<T>(&mut self) -> KeyValueMutator<T>
    where
        T: Traverser;

    fn iter_recursive<T>(&self) -> KeyValueIter<T>
    where
        T: Traverser;

    fn mutate_recursive<T>(&mut self) -> KeyValueMutator<T>
    where
        T: Traverser;
}

impl Iter for Value {
    fn iter<T>(&self) -> KeyValueIter<T>
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

    fn mutate<T>(&mut self) -> KeyValueMutator<T>
    where
        T: Traverser,
    {
        let mut traverser = T::new();
        traverser.set_depth(1);
        traverser.set_limit(None);
        KeyValueMutator {
            inner: self,
            traverser,
        }
    }

    fn iter_recursive<T>(&self) -> KeyValueIter<T>
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

    fn mutate_recursive<T>(&mut self) -> KeyValueMutator<T>
    where
        T: Traverser,
    {
        let mut traverser = T::new();
        traverser.set_depth(None);
        traverser.set_limit(None);
        KeyValueMutator {
            inner: self,
            traverser,
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::index;
    use crate::iter::dfs::Dfs;
    use crate::test::CollectCloned;
    use pretty_assertions::assert_eq;
    use serde_json::json;

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
                (index!(), value),
                (index!("person1"), json!({ "name": "bob" })),
                (index!("person1", "name"), json!("bob")),
                (index!("person2"), json!({ "name": "john" })),
                (index!("person2", "name"), json!("john")),
            ]
        );
    }
}
