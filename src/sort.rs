use itertools::Itertools;
use serde_json::{Map, Value};

pub trait Sort {
    // : SortInPlace {
    // /// Method use to merge two Json Values : ValueA <- ValueB.
    // fn merge(&mut self, other: Value);
    // /// Merge a new value in specific json pointer. If the field can't be merge in the specific
    // /// path, it raise an error.
    // fn merge_in(&mut self, json_pointer: &str, new_json_value: Value) -> io::Result<()>;
    pub fn sort(&mut self) {
        self.0 = sort_json_value_iter(self.0.clone());
    }
}

// impl MergeInPlace for serde_json::Value {
// }

impl Sort for serde_json::Value {}

fn sort_json_value_iter(value: Value) -> Value {
    let mut result = value.clone();
    for idx in value.iter_indices() {
        match value.get_path(&idx) {
            Some(Value::Object(v)) => {
                let sorted: Vec<(String, Value)> = v
                    .iter()
                    .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
                    .map(|(key, value)| (key.clone(), value.clone()))
                    .collect();
                match result.get_path_mut(&idx) {
                    Some(Value::Object(ref mut r)) => {
                        *r = Map::from_iter(sorted);
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
    result
}

fn sort_json_value_rec(value: &mut Value) {
    match value {
        &mut Value::Object(ref mut a) => {
            let sorted: Vec<(String, Value)> = a
                .iter_mut()
                .sorted_by(|a, b| Ord::cmp(&b.0, &a.0))
                .map(|(key, value)| {
                    sort_json_value_rec(value);
                    (key.clone(), value.clone())
                })
                .collect();
            *a = Map::from_iter(sorted);
        }
        _ => {}
    }
}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::Result;
    // use itertools::Itertools;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    #[test]
    fn test_sort_json_keys_recursively() -> Result<()> {
        let value: Value = json!({
            "title": "This is a title",
            "person" : {
                "firstName" : "John",
                "lastName" : "Doe"
            },
            "cities": [ "london", "paris" ]
        });
        Ok(())
    }
}
