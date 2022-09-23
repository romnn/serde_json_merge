#![allow(warnings)]
// #![feature(assert_matches)]

// pub mod sort;
pub mod index;
pub mod iter;
#[cfg(test)]
mod test;
mod utils;

pub use index::{Index, IndexRef, Path as IndexPath};
// pub mod test {
//     #![feature(assert_matches)]
// }
//
// use itertools::Itertools;
// use serde_json::Value;
// use std::collections::VecDeque;
// use std::rc::Rc;

// pub trait Index: serde_json::value::Index + std::fmt::Debug {}
// impl Index for String {}
// impl Index for str {}
// impl Index for usize {}

// pub type IndexRef = Rc<dyn Index>;

// pub struct JsonIndexIter<'a> {
//     inner: &'a serde_json::Value,
//     queue: VecDeque<Vec<IndexRef>>,
// }

// pub trait JsonIndex {
//     fn get_path<'i, 'a, I>(&'a self, indices: I) -> Option<&'a Value>
//     where
//         I: IntoIterator<Item = &'i IndexRef>;

//     fn get_path_mut<'i, 'a, I>(&'a mut self, indices: I) -> Option<&'a mut Value>
//     where
//         I: IntoIterator<Item = &'i IndexRef>;

//     fn iter_indices<'a>(&'a self) -> JsonIndexIter<'a>;
// }

// impl JsonIndex for Value {
//     fn get_path<'i, 'a, I>(&'a self, indices: I) -> Option<&'a Value>
//     where
//         I: IntoIterator<Item = &'i IndexRef>,
//     {
//         let mut val: Option<&'a Value> = Some(self);
//         for index in indices.into_iter() {
//             val = val.and_then(|v| v.get(index.as_ref()));
//         }
//         val
//     }

//     fn get_path_mut<'i, 'a, I>(&'a mut self, indices: I) -> Option<&'a mut Value>
//     where
//         I: IntoIterator<Item = &'i IndexRef>,
//     {
//         let mut val: Option<&'a mut Value> = Some(self);
//         for index in indices.into_iter() {
//             val = val.and_then(|v| v.get_mut(index.as_ref()));
//         }
//         val
//     }

//     fn iter_indices<'a>(&'a self) -> JsonIndexIter<'a> {
//         let queue = VecDeque::from_iter([Vec::new()]);
//         JsonIndexIter { inner: self, queue }
//     }
// }

// impl<'a> Iterator for JsonIndexIter<'a> {
//     type Item = Vec<IndexRef>;

//     #[inline]
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.queue.pop_back() {
//             Some(indices) => {
//                 match self.inner.get_path(&indices) {
//                     Some(Value::Object(o)) => {
//                         self.queue.extend(o.keys().map(|key| {
//                             let mut indices = indices.clone();
//                             indices.push(Rc::new(key.clone()));
//                             indices
//                         }));
//                     }
//                     _ => {}
//                 }
//                 Some(indices)
//             }
//             None => None,
//         }
//     }
// }

// fn merge_json_values_iter<V1, V2, M>(acc: V1, new: V2, mode: M) -> Value
// where
//     V1: Into<Value>,
//     V2: Into<Value>,
//     M: Into<Option<Merge>>,
// {
//     let mode = mode.into().unwrap_or_default();
//     let acc = acc.into();
//     let new = new.into();
//     let mut result = acc.clone();

//     for idx in new.iter_indices() {
//         match (result.get_path_mut(&idx), new.get_path(&idx)) {
//             // add new fields when merging two objects
//             (Some(&mut Value::Object(ref mut res)), Some(&Value::Object(ref other))) => {
//                 for (k, v) in other {
//                     res.entry(k.clone()).or_insert(Value::Null);
//                 }
//             }
//             // objects always have precedence over all other types
//             (Some(Value::Object(_)), Some(_)) => {}
//             // objects always have precedence over all other types
//             (Some(res), Some(other @ &Value::Object(_))) => match mode {
//                 Merge::Union => {
//                     *res = other.clone();
//                 }
//                 _ => {}
//             },
//             // null has lowest precedence
//             (_, Some(&Value::Null)) => {}
//             // null has lowest precedence
//             (Some(res @ &mut Value::Null), Some(other)) => {
//                 *res = other.clone();
//             }
//             // res and other are not objects and other is not null
//             (Some(res), Some(other)) => match mode {
//                 Merge::Union => {
//                     *res = other.clone();
//                 }
//                 _ => {}
//             },
//             // other does not have this field
//             (Some(_), None) => {}
//             // only other has this field, add it to res
//             // this can only be the case at the root level
//             (None, Some(other)) => {
//                 assert!(idx.len() == 1);
//                 let split = idx.len() - 1;
//                 let (parent_idx, child_idx) = (&idx[..split], &idx[split]);
//                 if let Some(parent) = result.get_path_mut(parent_idx) {
//                     let mut v = child_idx.index_or_insert(parent);
//                     *v = other.clone();
//                 }
//             }
//             // this should never occur
//             (None, None) => {}
//         }
//     }
//     result
// }

// fn merge_json_values_rec<M>(acc: &mut Value, new: &Value, mode: M)
// where
//     M: Into<Option<Merge>>,
// {
//     let mode = mode.into().unwrap_or_default();
//     match (acc, new) {
//         (&mut Value::Object(ref mut a), &Value::Object(ref n)) => {
//             for (k, v) in n {
//                 // get or create value for key in accumulator
//                 let k = a.entry(k.clone()).or_insert(Value::Null);
//                 merge_json_values_rec(k, v, mode);
//             }
//         }
//         // objects always have precedence over all other types
//         (&mut Value::Object(_), _) => {}
//         // objects always have precedence over all other types
//         (acc, new @ &Value::Object(_)) => match mode {
//             Merge::Union => {
//                 *acc = new.clone();
//             }
//             _ => {}
//         },
//         // null has lowest precedence
//         (_, &Value::Null) => {}
//         // null has lowest precedence
//         (acc @ &mut Value::Null, other) => {
//             *acc = other.clone();
//         }
//         // acc and other are not objects and other is not null
//         (acc, other) => match mode {
//             Merge::Union => {
//                 *acc = other.clone();
//             }
//             _ => {}
//         },
//     }
// }

// type JsonMap = serde_json::Map<String, Value>;

// fn json_union_rec<V>(values: impl Iterator<Item = V>) -> Value
// where
//     V: Into<Value>,
// {
//     let mut merged = Value::Object(Default::default());
//     for v in values {
//         merge_json_values_rec(&mut merged, &v.into(), Merge::Union);
//     }
//     merged
// }

// fn json_union_iter<V>(values: impl Iterator<Item = V>) -> Value
// where
//     V: Into<Value>,
// {
//     let mut merged = Value::Object(Default::default());
//     for mut v in values {
//         merged = merge_json_values_iter(merged, v.into(), Merge::Union);
//     }
//     merged
// }

// #[derive(Eq, PartialEq, Clone, Debug)]
// pub struct JsonValue(Value);

// impl Into<Value> for JsonValue {
//     fn into(self: Self) -> Value {
//         self.0
//     }
// }

// impl From<Value> for JsonValue {
//     fn from(value: Value) -> Self {
//         Self(value)
//     }
// }

// impl Into<Option<JsonMap>> for JsonValue {
//     fn into(self: Self) -> Option<JsonMap> {
//         match self.0 {
//             Value::Object(map) => Some(map),
//             _ => None,
//         }
//     }
// }

// impl From<JsonMap> for JsonValue {
//     fn from(map: JsonMap) -> Self {
//         Self(Value::Object(map))
//     }
// }

// impl std::ops::Deref for JsonValue {
//     type Target = Value;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// impl std::ops::DerefMut for JsonValue {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }

// impl JsonValue {
//     pub fn sort(&mut self) {
//         self.0 = sort_json_value_iter(self.0.clone());
//     }

//     pub fn from_union<V>(values: impl Iterator<Item = V>) -> Self
//     where
//         V: Into<Value>,
//     {
//         json_union_iter(values).into()
//     }

//     pub fn union(self, other: Value) -> Self {
//         merge_json_values_iter(self.0, other, Merge::Union).into()
//     }

//     pub fn merge(self, other: Value) -> Self {
//         merge_json_values_iter(self.0, other, Merge::Merge).into()
//     }
// }
