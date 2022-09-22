use itertools::Itertools;
use serde_json::{Map, Value};

pub trait Sort {
    pub fn sort(&mut self) {
        self.0 = sort_json_value_iter(self.0.clone());
    }
}

impl Sort for serde_json::Value {}

#[cfg(test)]
pub mod test {
    use super::*;
    use anyhow::Result;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    #[test]
    fn test_iter_json_keys_recursively() -> Result<()> {
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
