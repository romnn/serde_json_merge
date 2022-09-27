// use crate::index::{Index, Path as IndexPath};
// use crate::iter::{Dfs, Iter, Traverser};
// use serde_json::{Map, Value};
// use std::borrow::Borrow;

// // /// Method use to merge two Json Values : ValueA <- ValueB.

// trait Union: Sized {
//     fn union_all<T, V>(iter: impl IntoIterator<Item = V>) -> Self
//     where
//         T: Traverser,
//         V: Borrow<Value>;

//     fn union_all_by<T, V, F>(iter: impl IntoIterator<Item = V>, union: &mut F) -> Self
//     where
//         T: Traverser,
//         V: Borrow<Value>,
//         F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool;

//     fn union<T>(&mut self, other: &Self)
//     where
//         T: Traverser;

//     fn union_recursive<T>(&mut self, other: &Self)
//     where
//         T: Traverser;

//     #[inline]
//     fn into_union<T>(mut self, other: &Self) -> Self
//     where
//         T: Traverser,
//     {
//         self.union::<T>(other);
//         self
//     }

//     #[inline]
//     fn into_union_recursive<T>(mut self, other: &Self) -> Self
//     where
//         T: Traverser,
//     {
//         self.union_recursive::<T>(other);
//         self
//     }
// }

// trait Merge: Sized {
//     fn merge<T>(&mut self, other: &Self)
//     where
//         T: Traverser;

//     fn merge_recursive<T>(&mut self, other: &Self)
//     where
//         T: Traverser;

//     fn merge_by<T, F>(&mut self, other: &Self, merge: &mut F)
//     where
//         T: Traverser,
//         F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool;

//     fn merge_by_recursive<T, F>(&mut self, other: &Self, merge: &mut F)
//     where
//         T: Traverser,
//         F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool;

//     #[inline]
//     fn merged<T>(mut self, other: &Self) -> Self
//     where
//         T: Traverser,
//     {
//         self.merge::<T>(other);
//         self
//     }

//     #[inline]
//     fn merged_recursive<T>(mut self, other: &Self) -> Self
//     where
//         T: Traverser,
//     {
//         self.merge_recursive::<T>(other);
//         self
//     }

//     #[inline]
//     fn merged_by<T, F>(mut self, other: &Self, merge: &mut F) -> Self
//     where
//         T: Traverser,
//         F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool,
//     {
//         self.merge_by::<T, F>(other, merge);
//         self
//     }

//     #[inline]
//     fn merged_by_recursive<T, F>(mut self, other: &Self, merge: &mut F) -> Self
//     where
//         T: Traverser,
//         F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool,
//     {
//         self.merge_by_recursive::<T, F>(other, merge);
//         self
//     }
// }

// impl Union for Value {
//     #[inline]
//     fn union_all<T, V>(values: impl IntoIterator<Item = V>) -> Self
//     where
//         T: Traverser,
//         V: Borrow<Value>,
//     {
//         Self::union_all_by::<T, V, _>(values, &mut union_func)
//     }

//     #[inline]
//     fn union_all_by<T, V, F>(values: impl IntoIterator<Item = V>, union: &mut F) -> Self
//     where
//         T: Traverser,
//         V: Borrow<Value>,
//         F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool,
//     {
//         let mut result = Value::Object(Default::default());
//         for v in values.into_iter() {
//             result.merge_by_recursive::<T, F>(v.borrow(), union);
//         }
//         result
//     }

//     #[inline]
//     fn union<T>(&mut self, other: &Self)
//     where
//         T: Traverser,
//     {
//         self.merge_by::<T, _>(other, &mut union_func);
//     }

//     #[inline]
//     fn union_recursive<T>(&mut self, other: &Self)
//     where
//         T: Traverser,
//     {
//         self.merge_by_recursive::<T, _>(other, &mut union_func);
//     }
// }

// impl Merge for Value {
//     #[inline]
//     fn merge<T>(&mut self, other: &Self)
//     where
//         T: Traverser,
//     {
//         self.merge_by::<T, _>(other, &mut merge_func);
//     }

//     #[inline]
//     fn merge_recursive<T>(&mut self, other: &Self)
//     where
//         T: Traverser,
//     {
//         self.merge_by_recursive::<T, _>(other, &mut merge_func);
//     }

//     #[inline]
//     fn merge_by<T, F>(&mut self, other: &Self, merge: &mut F)
//     where
//         T: Traverser,
//         F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool,
//     {
//         let mut traverser = T::new();
//         traverser.set_limit(None);
//         traverser.set_depth(1);
//         while let Some(_) = traverser.process_next(other, |idx, new_value| {
//             if let Some(value) = self.get_index_mut(idx) {
//                 merge(idx, value, new_value)
//             } else {
//                 true
//             }
//         }) {}
//     }

//     #[inline]
//     fn merge_by_recursive<T, F>(&mut self, other: &Self, merge: &mut F)
//     where
//         T: Traverser,
//         F: FnMut(&IndexPath, &mut Value, Option<&Value>) -> bool,
//     {
//         let mut traverser = T::new();
//         traverser.set_limit(None);
//         traverser.set_depth(None);
//         while let Some(_) = traverser.process_next(other, |idx, new_value| {
//             if let Some(value) = self.get_index_mut(idx) {
//                 merge(idx, value, new_value)
//             } else {
//                 true
//             }
//         }) {}
//     }
// }

// fn union_func(idx: &IndexPath, this: &mut Value, other: Option<&Value>) -> bool {
//     match (this, other) {
//         // add new fields when merging two objects
//         (&mut Value::Object(ref mut res), Some(&Value::Object(ref other))) => {
//             for (k, v) in other {
//                 res.entry(k.clone()).or_insert(Value::Null);
//             }
//             true
//         }
//         // extend array with other array
//         (&mut Value::Array(ref mut this), Some(&Value::Array(ref other))) => {
//             this.extend(other.clone());
//             false
//         }
//         // never merge in NULL
//         (_, Some(&Value::Null)) => false,
//         // always merge NULL values
//         (this @ &mut Value::Null, Some(other)) => {
//             *this = other.clone();
//             false
//         }
//         _ => false,
//     }
// }

// fn merge_func(idx: &IndexPath, this: &mut Value, other: Option<&Value>) -> bool {
//     match (this, other) {
//         // add new fields when merging two objects
//         (&mut Value::Object(ref mut this), Some(&Value::Object(ref other))) => {
//             for (k, v) in other {
//                 this.entry(k.clone()).or_insert(Value::Null);
//             }
//             true
//         }
//         // extend array with other array
//         (&mut Value::Array(ref mut this), Some(&Value::Array(ref other))) => {
//             this.extend(other.clone());
//             false
//         }
//         // extend array with other value
//         (&mut Value::Array(ref mut this), Some(other)) => {
//             this.extend([other.clone()]);
//             false
//         }
//         // do not overwrite anything with null
//         (_, Some(&Value::Null)) => false,
//         // overwrite this with other
//         (this, Some(other)) => {
//             *this = other.clone();
//             false
//         }
//         _ => false,
//     }
// }

// #[cfg(test)]
// pub mod test {
//     use super::*;
//     use itertools::Itertools;
//     use pretty_assertions::assert_eq;
//     use serde_json::{json, Value};

//     #[test]
//     fn union_all_custom_func() {
//         let items = [
//             json!({
//                 "title": "This is a title",
//                 "person" : {
//                     "firstName" : "John",
//                     "lastName" : "Doe"
//                 },
//                 "cities": [ "london", "paris" ]
//             }),
//             json!({
//                 "person" : {
//                     "firstName" : "John",
//                 },
//             }),
//             json!({
//                 "another" : 1,
//             }),
//             json!({
//                 "another" : {
//                     "another" : 2,
//                 },
//             }),
//             json!({
//                 "person" : {
//                     "firstName" : {
//                         "test": "John",
//                     },
//                     "gender" : {
//                         "important": false,
//                     },
//                 },
//             }),
//         ];

//         let mut expected = json!({
//             "title": "This is a title",
//             "person" : {
//                 "lastName" : "Doe",
//                 "firstName" : {
//                     "test": "John",
//                 },
//                 "gender" : {
//                     "important": false,
//                 },
//             },
//             "cities": [ "london", "paris" ],
//             "another" : {
//                 "another" : 2,
//             },
//         });

//         use crate::sort::SortKeys;
//         expected.sort_keys_recursive::<Dfs>();

//         let mut custom_union_func =
//             |idx: &IndexPath, this: &mut Value, other: Option<&Value>| -> bool {
//                 match (this, other) {
//                     // add new fields when merging two objects
//                     (&mut Value::Object(ref mut this), Some(&Value::Object(ref other))) => {
//                         for (k, v) in other {
//                             this.entry(k.clone()).or_insert(Value::Null);
//                         }
//                         true
//                     }
//                     // never overwrite objects
//                     (Value::Object(_), Some(_)) => false,
//                     // always overwrite non-objects with objects
//                     (this, Some(other @ &Value::Object(_))) => {
//                         *this = other.clone();
//                         false
//                     }
//                     // never overwrite with NULL
//                     (_, Some(&Value::Null)) => false,
//                     // always overwrite NULL
//                     (this @ &mut Value::Null, Some(other)) => {
//                         *this = other.clone();
//                         false
//                     }
//                     // this and other are not objects and other is not NULL
//                     (this, Some(other)) => {
//                         *this = other.clone();
//                         false
//                     }
//                     _ => false,
//                 }
//             };

//         let len = items.len();
//         for perm in items.into_iter().permutations(len) {
//             // dbg!(&perm);
//             // let perm = perm.into_iter();
//             // let mut merged_rec: JsonValue = json_union_rec(perm.clone()).into();
//             // let mut merged_iter: JsonValue = json_union_iter(perm.clone()).into();
//             // merged_rec.sort();
//             // merged_iter.sort();
//             let mut union: Value = Union::union_all_by::<Dfs, _, _>(&perm, &mut custom_union_func);
//             union.sort_keys_recursive::<Dfs>();
//             assert_eq!(&union, &expected);
//             // assert_eq!(merged_iter, expected);
//             // assert_eq!(merged_rec, expected);
//             // assert_eq!(merged_rec, merged_iter);
//         }
//     }

//     #[test]
//     fn merge_array_string() {
//         // this is merge values
//         let base = json!(["a", "b"]);
//         let merge = json!(["b", "c"]);
//         assert_eq!(
//             &base.merged_recursive::<Dfs>(&merge),
//             &json!(["a", "b", "b", "c"])
//         );
//     }

//     #[test]
//     fn merge_array_object() {
//         let base = json!([{"value": "a"}, {"value": "b"}]);
//         let merge = json!([{"value": "b"}, {"value": "c"}]);
//         dbg!(base.clone().merged_recursive::<Dfs>(&merge));
//         assert_eq!(
//             &base.merged_recursive::<Dfs>(&merge),
//             &json!([
//                 {"value": "a"},
//                 {"value": "b"},
//                 {"value": "b"},
//                 {"value": "c"}
//             ])
//         );
//     }

//     #[test]
//     fn merge_object() {
//         let base = json!({"value1": "a", "value2": "b"});
//         let merge = json!({"value1": "a", "value2": "c", "value3": "d"});
//         assert_eq!(
//             &base.merged_recursive::<Dfs>(&merge),
//             &json!({
//                 "value1": "a",
//                 "value2": "c",
//                 "value3": "d",
//             })
//         );
//     }

//     #[test]
//     fn merge_string() {
//         let base = json!("a");
//         let merge = json!("b");
//         assert_eq!(&base.merged_recursive::<Dfs>(&merge), &merge);
//     }

//     #[test]
//     fn merge_complex() {
//         let base = json!({
//             "title": "This is a title",
//             "person" : {
//                 "firstName" : "John",
//                 "lastName" : "Doe"
//             },
//             "cities": [ "london", "paris" ]
//         });
//         let merge = json!({
//             "title": "",
//             "person" : {
//                 "firstName" : "",
//                 "lastName" : "",
//                 "new" : "this field is new"
//             },
//             "cities": [ "london" ],
//             "new": [ "this is new" ],
//         });

//         let expected = json!({
//             "title": "This is a title",
//             "person" : {
//                 "firstName" : "John",
//                 "lastName" : "Doe",
//                 "new" : "this field is new"
//             },
//             "cities": [ "london", "paris", "london" ],
//             "new": [ "this is new" ],
//         });
//         assert_eq!(&base.into_union_recursive::<Dfs>(&merge), &expected);
//     }
// }
