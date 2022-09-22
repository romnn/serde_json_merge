use itertools::Itertools;
use serde_json::{Map, Value};
use std::borrow::Borrow;
use std::rc::Rc;

pub type IndexRef = Rc<dyn serde_json::value::Index>;

pub trait Index {
    fn get_index<'a, I>(&'a self, indices: I) -> Option<&'a Value>
    where
        I: IntoIterator, // <Item = IndexRef>,
        I::Item: Borrow<IndexRef>;
    // I: IntoIterator<Item = &'i IndexRef>;

    fn get_index_mut<'a, I>(&'a mut self, indices: I) -> Option<&'a mut Value>
    where
        I: IntoIterator, // <Item = IndexRef>,
        I::Item: Borrow<IndexRef>;
}

impl Index for serde_json::Value {
    fn get_index<'a, I>(&'a self, indices: I) -> Option<&'a Value>
    where
        I: IntoIterator, // <Item = &'i IndexRef>,
        I::Item: Borrow<IndexRef>,
    {
        let mut val: Option<&'a Value> = Some(self);
        for index in indices.into_iter() {
            val = val.and_then(|v| v.get(index.borrow().as_ref()));
        }
        val
    }

    fn get_index_mut<'i, 'a, I>(&'a mut self, indices: I) -> Option<&'a mut Value>
    where
        I: IntoIterator, // <Item = &'i IndexRef>,
        I::Item: Borrow<IndexRef>,
        // I: IntoIterator<Item = &'i IndexRef>,
    {
        let mut val: Option<&'a mut Value> = Some(self);
        for index in indices.into_iter() {
            val = val.and_then(|v| v.get_mut(index.borrow().as_ref()));
        }
        val
    }
}

// macro_rules! get {
//     ( $value:expr, $( $idx:expr ),* ) => {
//         $value.get(
//     }
// }

macro_rules! index {
    ( $( $idx:expr ),* ) => {
        {
            let mut index: Vec<$crate::index::IndexRef> = Vec::new();
            // let mut val: Option<&Value> = Some($val);
            $(
                // index.push(std::rc::Rc::new($idx ).as_ref() as &serde_json::value::Index);
                use std::rc::Rc;
                // let idx: $crate::index::IndexRef = Rc::new($idx);
                index.push(Rc::new($idx));
            )*
            index
        }
    };
}

#[cfg(test)]
pub mod test {
    use super::Index;
    use super::*;
    use anyhow::Result;
    use lazy_static::lazy_static;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    lazy_static! {
        static ref COMPLEX_JSON: Value = json!({
            "string": "value",
            "bool": true,
            "null": null,
            "number": 1,
            "object" : {
                "string": "value",
                "bool": true,
                "null": null,
                "number": 1,
                "object" : {},
            },
            "array": [
                "value",
                true,
                null,
                1,
                {
                    "string": "value",
                    "bool": true,
                    "null": null,
                    "number": 1,
                    "object" : {},
                    "array" : [],
                },
                [
                    "value",
                    true,
                    null,
                    1,
                    {
                        "string": "value",
                        "bool": true,
                        "null": null,
                        "number": 1,
                        "object" : {},
                        "array" : [],
                    },
                ]
            ]
        });
    }

    macro_rules! string {
        ( $value:expr ) => {{
            Some(&serde_json::Value::String($value.into()))
        }};
    }

    macro_rules! mut_string {
        ( $value:expr ) => {{
            Some(&mut serde_json::Value::String($value.into()))
        }};
    }

    #[test]
    fn test_get_index() -> Result<()> {
        let value = COMPLEX_JSON.clone();
        assert_eq!(value.get_index(index!("string")), string!("value"),);
        Ok(())
    }

    #[test]
    fn test_get_index_mut() -> Result<()> {
        let mut value = COMPLEX_JSON.clone();
        assert_eq!(value.get_index_mut(index!("string")), mut_string!("value"),);
        Ok(())
    }
}
