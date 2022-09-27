#![allow(warnings)]

pub mod index;
pub mod iter;
pub mod merge;
pub mod sort;
#[cfg(test)]
mod test;
mod utils;

pub use index::{Index, IndexRef, Path as IndexPath};
pub use iter::{Dfs, DfsIter, Iter};
pub use merge::{Merge, Union};
pub use sort::Sort;
