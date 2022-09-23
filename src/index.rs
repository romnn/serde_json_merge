use super::utils;
use fancy_regex::Regex;
use itertools::Itertools;
use serde_json::{Map, Value};
use std::borrow::Borrow;
use std::rc::Rc;

pub type IndexRef = Rc<dyn serde_json::value::Index>;

#[derive(Clone, Default)]
pub struct Path(Vec<IndexRef>);

impl IntoIterator for Path {
    type Item = IndexRef;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Path {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, index: impl serde_json::value::Index + 'static) {
        self.0.push(Rc::new(index));
    }
}

impl std::ops::Index<Path> for Value {
    type Output = Value;

    fn index(&self, index: Path) -> &Self::Output {
        static NULL: Value = Value::Null;
        self.get_index(index).unwrap_or(&NULL)
    }
}

// impl std::ops::Index<std::ops::Range<usize>> for Path {
//     type Output = Path;

//     fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
//         Self::from_iter(self.0[index.start..index.end].iter().cloned())
//         // &SliceWrapper {slice: self.vec[index]}
//     }
// }

impl std::ops::Deref for Path {
    type Target = Vec<IndexRef>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Path {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

// impl std::ops::Index<Path> for Value {
//     type Output = Value;

//     fn index(&self, index: Path) -> &Self::Output {
//         static NULL: Value = Value::Null;
//         self.get_index(index).unwrap_or(&NULL)
//     }
// }

// impl std::ops::IndexMut<Path> for Value {
//     fn index(&self, index: Path) -> &mut Self::Output {
//         static NULL: Value = Self::Output::Null;
//         // index.as_ref().index_into(self).unwrap_or(&NULL)
//         todo!()
//         // self.get_index(index).unwrap_or(&NULL)
//         // .index_into(self)
//     }
// }

pub trait Index {
    fn get_path<'a, S>(&'a self, path: S) -> Option<&'a Value>
    where
        S: Borrow<str>;

    fn get_path_mut<'a, S>(&'a mut self, path: S) -> Option<&'a mut Value>
    where
        S: Borrow<str>;

    fn get_path_iter<'a, P>(&'a self, path_iter: P) -> Option<&'a Value>
    where
        P: IntoIterator,
        P::Item: Borrow<str>;

    fn get_path_iter_mut<'a, P>(&'a mut self, path_iter: P) -> Option<&'a mut Value>
    where
        P: IntoIterator,
        P::Item: Borrow<str>;

    fn get_index<'a, I>(&'a self, indices: I) -> Option<&'a Value>
    where
        I: IntoIterator,
        I::Item: Borrow<IndexRef>;

    fn get_index_mut<'a, I>(&'a mut self, indices: I) -> Option<&'a mut Value>
    where
        I: IntoIterator,
        I::Item: Borrow<IndexRef>;
}

lazy_static::lazy_static! {
    // pub static ref SPLIT_PATH_REGEX: Regex = Regex::new(r"(?<!\\)\/").unwrap();
    // (?<!AU)\$(\d+)
    pub static ref SPLIT_PATH_REGEX: Regex = Regex::new(
        // r"(?<!\\)\/"
        // r"^((.*)(?<!\\/)\/)(.*)$"
        r"((^/)|((?<!\\)/))"
    ).unwrap();
    // pub static ref SPLIT_PATH_REGEX: Regex = Regex::new(r"[^\\]/").unwrap();
}

impl Index for Value {
    fn get_index<'a, I>(&'a self, indices: I) -> Option<&'a Value>
    where
        I: IntoIterator,
        I::Item: Borrow<IndexRef>,
    {
        let mut val: Option<&'a Value> = Some(self);
        for index in indices.into_iter() {
            val = match val {
                Some(v) => v.get(index.borrow().as_ref()),
                None => return None,
            };
        }
        val
    }

    fn get_index_mut<'a, I>(&'a mut self, indices: I) -> Option<&'a mut Value>
    where
        I: IntoIterator,
        I::Item: Borrow<IndexRef>,
    {
        let mut val: Option<&'a mut Value> = Some(self);
        for index in indices.into_iter() {
            val = match val {
                Some(v) => v.get_mut(index.borrow().as_ref()),
                None => return None,
            };
        }
        val
    }

    fn get_path_iter<'a, P>(&'a self, path_iter: P) -> Option<&'a Value>
    where
        P: IntoIterator,
        P::Item: Borrow<str>,
    {
        let mut val: Option<&'a Value> = Some(self);
        for str_index in path_iter.into_iter() {
            let str_index = str_index.borrow();
            match val {
                Some(Value::Array(_)) => {
                    if let Ok(arr_idx) = str_index.parse::<usize>() {
                        val = val.and_then(|v| v.get(arr_idx));
                    }
                }
                None => return None,
                _ => {}
            };
            val = val.and_then(|v| v.get(&str_index));
        }
        val
    }

    fn get_path_iter_mut<'a, P>(&'a mut self, path_iter: P) -> Option<&'a mut Value>
    where
        P: IntoIterator,
        P::Item: Borrow<str>,
    {
        let mut val: Option<&'a mut Value> = Some(self);
        for str_index in path_iter.into_iter() {
            let str_index = str_index.borrow();
            match val {
                Some(Value::Array(_)) if is_integer(str_index) => {
                    if let Ok(arr_idx) = str_index.parse::<usize>() {
                        val = val.and_then(|v| v.get_mut(arr_idx));
                    }
                }
                None => return None,
                _ => {}
            };
            val = val.and_then(|v| v.get_mut(&str_index));
        }
        val
    }

    fn get_path<'a, P>(&'a self, path: P) -> Option<&'a Value>
    where
        P: Borrow<str>,
    {
        self.get_path_iter(split_path(path.borrow()))
    }

    fn get_path_mut<'a, P>(&'a mut self, path: P) -> Option<&'a mut Value>
    where
        P: Borrow<str>,
    {
        self.get_path_iter_mut(split_path(path.borrow()))
    }
}

pub fn split_path<'b>(path: &'b str) -> impl Iterator<Item = &'b str> + 'b {
    // pub fn split_path<'b>(path: &'b str) -> Vec<&'b str> {
    // (?<!\\)\/
    // SPLIT_PATH_REGEX.split(path.borrow()) // .collect()
    // let test = SPLIT_PATH_REGEX.captures(path);
    let finder = SPLIT_PATH_REGEX.find_iter(path);
    let iter = utils::Split::new(finder).filter(|cap| cap.trim().len() > 0);
    iter
    // dbg!(&test);
    // for t in test {
    //     if let Ok(t) = t {
    //         dbg!(t.as_str());
    //     }
    // }
    // // dbg!(test.iter().collect::<Vec<Options>>());
    // vec![]
    // split(path) // .collect()
    // let test = path.into();
    // let out: Vec<&str> = vec![];
    // SPLIT_PATH_REGEX.captures(path).map(|cap| {
    //     path[cap.start() + 1
    // })// .collect()
}

pub fn is_integer(s: impl Borrow<str>) -> bool {
    lazy_static::lazy_static! {
        pub static ref IS_QUOTED_REGEX: Regex = Regex::new(
            r"^\d+$"
        ).unwrap();
    }
    IS_QUOTED_REGEX.is_match(s.borrow()).unwrap_or(false)
}

macro_rules! index {
    ( $( $idx:expr ),* ) => {{
        let mut index = $crate::index::Path::new();
        $(
            index.push($idx);
        )*
        index
    }};
}

#[cfg(test)]
pub mod test {
    use super::*;
    use crate::test::{assert_matches, ValueExt};
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
            "0": 1,
            "object" : {
                "string": "value",
                "bool": true,
                "null": null,
                "number": 1,
                "object" : {},
                "array": [],
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

    macro_rules! get_index_tests {
        ($name:ident: $val:ident { $($path:literal: $index:expr => $expected:expr,)* }) => {
            #[test]
            fn $name() -> Result<()> {
                let mut value = $val.clone();
                $(
                    let expected = Option::from($expected);
                    assert_eq!(
                        value.get_index($index),
                        expected.as_ref(), $path);
                    // assert_eq!(
                    //     value.get_path($path),
                    //     expected.as_ref(), $path);
                )*
                Ok(())
            }

            paste::item! {
                #[test]
                pub fn [< $name _ mut >]() -> Result<()> {
                    let mut value = $val.clone();
                    $(
                        let mut expected = Option::from($expected);
                        assert_eq!(
                            value.get_index_mut($index),
                            expected.as_mut(), $path);
                        // assert_eq!(
                        //     value.get_path_mut($path),
                        //     expected.as_mut(), $path);
                    )*
                    Ok(())
                }
            }
        }
    }

    get_index_tests!(test_complex_json_get_index: COMPLEX_JSON{
        "/string": index!("string") => Value::String("value".into()),
        "/bool": index!("bool") => Value::Bool(true),
        "/null": index!("null") => Value::Null,
        "/number": index!("number") => Value::Number(1.into()),
        "/0": index!("0") => Value::Number(1.into()),
        // "/0": index!(0) => None,
        "/object/string": index!("object", "string") => Value::String("value".into()),
        "/object/bool": index!("object", "bool") => Value::Bool(true),
        // "/object/array": index!("object", "array") => Value::Bool(true),
    });

    #[test]
    fn test_get_index() -> Result<()> {
        let mut value = COMPLEX_JSON.clone();
        assert_eq!(
            value.get_index(index!("string")),
            Some(&Value::String("value".into()))
        );
        assert_eq!(value.get_index(index!("bool")), Some(&Value::Bool(true)));
        assert_eq!(value.get_index(index!("null")), Some(&Value::Null));
        assert_eq!(
            value.get_index(index!("number")),
            Some(&Value::Number(1.into()))
        );
        assert_eq!(value.get_index(index!("0")), Some(&Value::Number(1.into())));
        assert_eq!(value.get_index(index!(0)), None);

        assert_matches!(value.get_index(index!("object")), Some(&Value::Object(_)));
        assert_eq!(
            value.get_index(index!("object", "string")),
            Some(&Value::String("value".into()))
        );
        assert_eq!(
            value.get_index(index!("object", "bool")),
            Some(&Value::Bool(true))
        );
        assert_matches!(
            value.get_index(index!("object", "array")),
            Some(&Value::Array(_))
        );
        assert_eq!(value.get_index(index!("object", "array", 0)), None,);
        assert_matches!(
            value.get_index(index!("object", "object")),
            Some(&Value::Object(_))
        );
        assert_eq!(value.get_index(index!("object", "object", "empty")), None);
        assert_eq!(value.get_index(index!("object", "object", 0)), None);
        assert_eq!(value.get_index(index!("object", 0)), None);

        assert_eq!(
            value.get_index(index!("array", 0)),
            Some(&Value::String("value".into()))
        );
        assert_eq!(
            value.get_index(index!("array", 1)),
            Some(&Value::Bool(true))
        );
        assert_eq!(value.get_index(index!("array", 100)), None);

        // assert_eq!(value.get_index_mut(index!("string")), mut_string!("value"),);
        // assert_eq!(value.get_index(["string"]), string("value"),);
        Ok(())
    }

    // #[test]
    // fn test_get_index_mut() -> Result<()> {
    //     let mut value = COMPLEX_JSON.clone();
    //     assert_eq!(value.get_index_mut(index!("string")), mut_string!("value"),);
    //     Ok(())
    // }

    #[test]
    fn test_split_path_regex() -> Result<()> {
        assert_eq!(split_path("test").collect::<Vec<&str>>(), vec!["test"]);
        assert_eq!(
            split_path("hello/world").collect::<Vec<&str>>(),
            vec!["hello", "world"]
        );
        assert_eq!(
            split_path("hello/0/world").collect::<Vec<&str>>(),
            vec!["hello", "0", "world"]
        );
        assert_eq!(
            split_path(r"hello/test\/test/world").collect::<Vec<&str>>(),
            vec!["hello", "test\\/test", "world"]
        );
        assert_eq!(
            split_path(r"hello/\/test 0 /world").collect::<Vec<&str>>(),
            vec!["hello", "\\/test 0 ", "world"]
        );
        assert_eq!(
            split_path(r"hello/\/test 0 /'0'").collect::<Vec<&str>>(),
            vec!["hello", "\\/test 0 ", "'0'"]
        );
        assert_eq!(
            split_path(r#"hello/\/test 0 /"0""#).collect::<Vec<&str>>(),
            vec!["hello", "\\/test 0 ", "\"0\""]
        );
        Ok(())
    }

    #[test]
    fn test_is_integer() {
        assert_eq!(is_integer(""), false);
        assert_eq!(is_integer(" "), false);
        assert_eq!(is_integer("0"), true);
        assert_eq!(is_integer("12"), true);
        assert_eq!(is_integer(" 12"), false);
        assert_eq!(is_integer("12 "), false);
        assert_eq!(is_integer(r#""12""#), false);
        assert_eq!(is_integer(r#"'12'"#), false);
        assert_eq!(is_integer(r#""12"#), false);
    }

    #[test]
    fn test_index_path_indexing() -> Result<()> {
        let value = json!({
            "0": 0,
            "1": 1,
            "2": 2,
            "3": 3,
        });
        let index = index!("0", "1", "2", "3");
        assert_eq!(index.len(), 4);
        assert_eq!(value[index[0].as_ref()], 0);
        assert_eq!(value[index[1].as_ref()], 1);
        assert_eq!(value[index[2].as_ref()], 2);
        Ok(())
    }

    #[test]
    fn test_index_path_slicing() -> Result<()> {
        let value = json!({
            "1": {
                "2": {
                    "3": [1, 2, 3]
                }
            }
        });
        let index = index!("1", "2", "3", 1);
        assert_eq!(
            value.get_index(&index[..0]).try_keys(),
            Some(vec!["1".into()])
        );
        assert_eq!(
            value.get_index(&index[..1]).try_keys(),
            Some(vec!["2".into()])
        );
        assert_eq!(
            value.get_index(&index[..2]).try_keys(),
            Some(vec!["3".into()])
        );
        assert_eq!(
            value.get_index(&index[..3]),
            Some(&Value::Array(
                vec![1, 2, 3]
                    .into_iter()
                    .map(|n| Value::Number(n.into()))
                    .collect()
            ))
        );
        assert_eq!(value.get_index(&index[..4]), Some(&Value::Number(2.into())));
        Ok(())
    }

    #[test]
    fn test_get_index_arguments() -> Result<()> {
        let mut value = COMPLEX_JSON.clone();
        // real indices
        value.get_index(index!("string"));
        value.get_index(index!("string".to_string()));
        value.get_index(index!("string", 0));
        value.get_index(index!("string".to_string(), 0, "test"));

        // string indices
        value.get_path_iter(["string"]);
        value.get_path_iter(["string".to_string()]);
        value.get_path_iter(vec!["string"]);
        value.get_path_iter(vec!["string", "0", "3"]);
        value.get_path_iter(vec!["string".to_string()]);
        value.get_path_iter(std::collections::VecDeque::from_iter(["string"]));

        let test = vec!["test"];
        value.get_path_iter(test.into_iter());
        Ok(())
    }
}
