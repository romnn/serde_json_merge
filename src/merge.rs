use crate::index::{Index, Path as IndexPath};
use crate::iter::{Dfs, Iter, Traverser};
use serde_json::{Map, Value};

// #[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// pub enum Type {
//     Merge,
//     Union,
// }

// impl Default for Type {
//     #[inline]
//     fn default() -> Self {
//         Self::Merge
//     }
// }

// merge keys: add new object keys that are in other but not in this
//
// merge values: merge array values of arrays that are present in both this and other and otherwise
// overwrite values in this with other
//
// m
//
// /// Method use to merge two Json Values : ValueA <- ValueB.

fn merge_keys_old(this: Option<&mut Value>, other: Option<&Value>) {
    match (this, other) {
        // add new fields when merging two objects
        (Some(&mut Value::Object(ref mut res)), Some(&Value::Object(ref other))) => {
            for (k, v) in other {
                res.entry(k.clone()).or_insert(Value::Null);
            }
        }
        // objects always have precedence over all other types
        (Some(Value::Object(_)), Some(_)) => {}
        // objects always have precedence over all other types
        (Some(res), Some(other @ &Value::Object(_))) => {
            // match mode {
            //     Merge::Union => {
            //         *res = other.clone();
            //     }
            //     _ => {}
            // }
        }
        // null has lowest precedence
        (_, Some(&Value::Null)) => {}
        // null has lowest precedence
        (Some(res @ &mut Value::Null), Some(other)) => {
            *res = other.clone();
        }
        // res and other are not objects and other is not null
        (Some(res), Some(other)) => {
            // match mode {
            //     Merge::Union => {
            //         *res = other.clone();
            //     }
            //     _ => {}
            // }
        }
        // other does not have this field
        (Some(_), None) => {}
        // only other has this field, add it to res
        // this can only be the case at the root level
        (None, Some(other)) => {
            todo!();
            // assert!(idx.len() == 1);
            // let split = idx.len() - 1;
            // let (parent_idx, child_idx) = (&idx[..split], &idx[split]);
            // if let Some(parent) = result.get_path_mut(parent_idx) {
            //     let mut v = child_idx.index_or_insert(parent);
            //     *v = other.clone();
            // }
        }
        // this should never occur
        (None, None) => {
            // unreachable!();
        }
    }
}

fn union(idx: &IndexPath, this: &mut Value, other: Option<&Value>) -> bool {
    match (this, other) {
        // add new fields when merging two objects
        (&mut Value::Object(ref mut res), Some(&Value::Object(ref other))) => {
            for (k, v) in other {
                res.entry(k.clone()).or_insert(Value::Null);
            }
            true
        }
        // objects always have precedence over all other types
        // (Value::Object(_), Some(_)) => {
        //     true
        // }
        // // objects always have precedence over all other types
        // (res, Some(other @ &Value::Object(_))) => {
        //     //     Merge::Union => {
        //     //         *res = other.clone();
        //     //     }
        //     //     _ => {}
        //     // }
        // }
        // null has lowest precedence
        (_, Some(&Value::Null)) => false,
        // null has lowest precedence
        (this @ &mut Value::Null, Some(other)) => {
            *this = other.clone();
            false
        }
        // res and other are not objects and other is not null
        // (res, Some(other)) => {
        //     // match mode {
        //     //     Merge::Union => {
        //     //         *res = other.clone();
        //     //     }
        //     //     _ => {}
        //     // }
        // }
        // other does not have this field
        // (Some(_), None) => {}
        // only other has this field, add it to res
        // this can only be the case at the root level
        // (None, Some(other)) => {
        //     todo!();
        //     // assert!(idx.len() == 1);
        //     // let split = idx.len() - 1;
        //     // let (parent_idx, child_idx) = (&idx[..split], &idx[split]);
        //     // if let Some(parent) = result.get_path_mut(parent_idx) {
        //     //     let mut v = child_idx.index_or_insert(parent);
        //     //     *v = other.clone();
        //     // }
        // }
        _ => {
            false
            // unreachable!();
        }
    }
}

// fn merge(this: Option<&mut Value>, other: Option<&Value>) -> bool {
fn merge(idx: &IndexPath, this: &mut Value, other: Option<&Value>) -> bool {
    // dbg!(&this, &other);
    match (this, other) {
        // add new fields when merging two objects
        (&mut Value::Object(ref mut this), Some(&Value::Object(ref other))) => {
            for (k, v) in other {
                this.entry(k.clone()).or_insert(Value::Null);
            }
            true
        }
        // extend array with other array
        (&mut Value::Array(ref mut this), Some(&Value::Array(ref other))) => {
            this.extend(other.clone());
            false
        }
        // extend array with other value
        (&mut Value::Array(ref mut this), Some(other)) => {
            this.extend([other.clone()]);
            false
        }
        // do not overwrite anything with null
        (_, Some(&Value::Null)) => false,
        // overwrite this with other
        (this, Some(other)) => {
            *this = other.clone();
            false
        }
        _ => false,
    }
}

trait Merge: Sized {
    // fn union(self, other: Value) -> Self;

    // fn merge_by(&mut self, other: Value);

    #[inline]
    fn merge_by_recursive(&mut self, other: &Self);

    #[inline]
    fn merged_by_recursive(mut self, other: &Self) -> Self {
        self.merge_by_recursive(other);
        self
    }

    // #[inline]
    // fn merged_by(mut self, other: Value) -> Self {
    //     self.merge_by(other);
    //     self
    // }
}

impl Merge for Value {
    // #[inline]
    // fn merge(&mut self, other: Value) {
    // }
    #[inline]
    fn merge_by_recursive(&mut self, other: &Self) {
        let mut traverser = Dfs::new();
        traverser.set_limit(None);
        traverser.set_depth(None);
        while let Some(_) = traverser.process_next(other, |idx, new_value| {
            if let Some(value) = self.get_index_mut(idx) {
                // merge(idx, value, new_value)
                union(idx, value, new_value)
            } else {
                true
            }
        }) {}
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    // #[test]
    // fn test_merge_json_union() -> Result<()> {
    //     let items: Vec<JsonValue> = [
    //         json!({
    //             "title": "This is a title",
    //             "person" : {
    //                 "firstName" : "John",
    //                 "lastName" : "Doe"
    //             },
    //             "cities": [ "london", "paris" ]
    //         }),
    //         json!({
    //             "person" : {
    //                 "firstName" : "John",
    //             },
    //         }),
    //         json!({
    //             "another" : 1,
    //         }),
    //         json!({
    //             "another" : {
    //                 "another" : 2,
    //             },
    //         }),
    //         json!({
    //             "person" : {
    //                 "firstName" : {
    //                     "test": "John",
    //                 },
    //                 "gender" : {
    //                     "important": false,
    //                 },
    //             },
    //         }),
    //     ]
    //     .into_iter()
    //     .map(Into::into)
    //     .collect();

    //     let mut expected: JsonValue = json!({
    //         "title": "This is a title",
    //         "person" : {
    //             "lastName" : "Doe",
    //             "firstName" : {
    //                 "test": "John",
    //             },
    //             "gender" : {
    //                 "important": false,
    //             },
    //         },
    //         "cities": [ "london", "paris" ],
    //         "another" : {
    //             "another" : 2,
    //         },
    //     })
    //     .into();
    //     expected.sort();

    //     let len = items.len();
    //     for perm in items.into_iter().permutations(len) {
    //         // dbg!(&perm);
    //         let perm = perm.into_iter();
    //         let mut merged_rec: JsonValue = json_union_rec(perm.clone()).into();
    //         let mut merged_iter: JsonValue = json_union_iter(perm.clone()).into();
    //         merged_rec.sort();
    //         merged_iter.sort();
    //         assert_eq!(merged_iter, expected);
    //         assert_eq!(merged_rec, expected);
    //         assert_eq!(merged_rec, merged_iter);
    //     }
    //     Ok(())
    // }

    #[test]
    fn it_should_merge_array_string() {
        // this is merge values
        let base = json!(["a", "b"]);
        let merge = json!(["b", "c"]);
        assert_eq!(
            &base.merged_by_recursive(&merge),
            &json!(["a", "b", "b", "c"])
        );
    }

    #[test]
    fn it_should_merge_array_object() {
        let base = json!([{"value": "a"}, {"value": "b"}]);
        let merge = json!([{"value": "b"}, {"value": "c"}]);
        dbg!(base.clone().merged_by_recursive(&merge));
        assert_eq!(
            &base.merged_by_recursive(&merge),
            &json!([
                {"value": "a"},
                {"value": "b"},
                {"value": "b"},
                {"value": "c"}
            ])
        );
    }

    #[test]
    fn it_should_merge_object() {
        let base = json!({"value1": "a", "value2": "b"});
        let merge = json!({"value1": "a", "value2": "c", "value3": "d"});
        assert_eq!(
            &base.merged_by_recursive(&merge),
            &json!({
                "value1": "a",
                "value2": "c",
                "value3": "d",
            })
        );
    }

    #[test]
    fn it_should_merge_string() {
        let base = json!("a");
        let merge = json!("b");
        assert_eq!(&base.merged_by_recursive(&merge), &merge);
    }

    #[test]
    fn test_merge() {
        let base = json!({
            "title": "This is a title",
            "person" : {
                "firstName" : "John",
                "lastName" : "Doe"
            },
            "cities": [ "london", "paris" ]
        });
        let merge = json!({
            "title": "",
            "person" : {
                "firstName" : "",
                "lastName" : "",
                "new" : "this field is new"
            },
            "cities": [ "london" ],
            "new": [ "this is new" ],
        });

        let expected = json!({
            "title": "This is a title",
            "person" : {
                "firstName" : "John",
                "lastName" : "Doe",
                "new" : "this field is new"
            },
            "cities": [ "london", "paris" ],
            "new": [ "this is new" ],
        });
        assert_eq!(&base.merged_by_recursive(&merge), &expected);
    }
}
