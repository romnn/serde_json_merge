use serde_json::{Map, Value};

#[derive(Clone, Copy, Debug)]
pub enum Merge {
    Merge,
    Union,
}

impl Default for Merge {
    fn default() -> Self {
        Self::Merge
    }
}

trait MergeInPlace {
    fn merge_in_place(&mut self, other: Value);
    // fn merge_in_place(&mut self, other: Value);

    /// Merge a new value in specific json pointer. If the field can't be merge in the specific
    /// path, it raise an error.
    // fn merge_in(&mut self, json_pointer: &str, new_json_value: Value) -> io::Result<()>;
}


pub trait Merge: MergeInPlace {
    // /// Method use to merge two Json Values : ValueA <- ValueB.
    // fn merge(&mut self, other: Value);
    // /// Merge a new value in specific json pointer. If the field can't be merge in the specific
    // /// path, it raise an error.
    // fn merge_in(&mut self, json_pointer: &str, new_json_value: Value) -> io::Result<()>;
}

impl MergeInPlace for serde_json::Value {
}

impl Merge for serde_json::Value {}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::Result;
    use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    #[test]
    fn test_merge_json_union() -> Result<()> {
        let items: Vec<JsonValue> = [
            json!({
                "title": "This is a title",
                "person" : {
                    "firstName" : "John",
                    "lastName" : "Doe"
                },
                "cities": [ "london", "paris" ]
            }),
            json!({
                "person" : {
                    "firstName" : "John",
                },
            }),
            json!({
                "another" : 1,
            }),
            json!({
                "another" : {
                    "another" : 2,
                },
            }),
            json!({
                "person" : {
                    "firstName" : {
                        "test": "John",
                    },
                    "gender" : {
                        "important": false,
                    },
                },
            }),
        ]
        .into_iter()
        .map(Into::into)
        .collect();

        let mut expected: JsonValue = json!({
            "title": "This is a title",
            "person" : {
                "lastName" : "Doe",
                "firstName" : {
                    "test": "John",
                },
                "gender" : {
                    "important": false,
                },
            },
            "cities": [ "london", "paris" ],
            "another" : {
                "another" : 2,
            },
        })
        .into();
        expected.sort();

        let len = items.len();
        for perm in items.into_iter().permutations(len) {
            // dbg!(&perm);
            let perm = perm.into_iter();
            let mut merged_rec: JsonValue = json_union_rec(perm.clone()).into();
            let mut merged_iter: JsonValue = json_union_iter(perm.clone()).into();
            merged_rec.sort();
            merged_iter.sort();
            assert_eq!(merged_iter, expected);
            assert_eq!(merged_rec, expected);
            assert_eq!(merged_rec, merged_iter);
        }
        Ok(())
    }

    #[test]
    fn test_merge_json() -> Result<()> {
        let base: JsonValue = json!({
            "title": "This is a title",
            "person" : {
                "firstName" : "John",
                "lastName" : "Doe"
            },
            "cities": [ "london", "paris" ]
        })
        .into();
        let merge: JsonValue = json!({
            "title": "",
            "person" : {
                "firstName" : "",
                "lastName" : "",
                "new" : "this field is new"
            },
            "cities": [ "london" ],
            "new": [ "this is new" ],
        })
        .into();
        let mut expected: JsonValue = json!({
            "title": "This is a title",
            "person" : {
                "firstName" : "John",
                "lastName" : "Doe",
                "new" : "this field is new"
            },
            "cities": [ "london", "paris" ],
            "new": [ "this is new" ],
        })
        .into();
        expected.sort();

        let mut merged_rec = base.clone();
        merge_json_values_rec(&mut merged_rec, &merge, Merge::Merge);
        let mut merged_rec: JsonValue = merged_rec.into();
        merged_rec.sort();

        let mut merged_iter: JsonValue =
            merge_json_values_iter(base.clone(), merge.clone(), Merge::Merge).into();
        merged_iter.sort();

        assert_eq!(merged_rec, expected);
        assert_eq!(merged_iter, expected);
        assert_eq!(merged_rec, merged_iter);
        Ok(())
    }
}
