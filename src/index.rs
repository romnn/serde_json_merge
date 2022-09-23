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

impl std::ops::Index<Path> for Value {
    type Output = Value;

    fn index(&self, index_path: Path) -> &Self::Output {
        static NULL: Value = Value::Null;
        self.get_index(index_path).unwrap_or(&NULL)
    }
}

impl std::ops::IndexMut<Path> for Value {
    fn index_mut<'a>(&'a mut self, index_path: Path) -> &'a mut Self::Output {
        let mut val: &'a mut Value = self;
        for index in index_path.into_iter() {
            val = index.as_ref().index_or_insert(val);
        }
        val
    }
}

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
    pub static ref SPLIT_PATH_REGEX: Regex = Regex::new(
        r"((^/)|((?<!\\)/))"
    ).unwrap();
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
                Some(Value::Array(_)) if is_integer(str_index) => {
                    if let Ok(arr_idx) = str_index.parse::<usize>() {
                        val = val.and_then(|v| v.get(arr_idx));
                        continue;
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
                        continue;
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
    let finder = SPLIT_PATH_REGEX.find_iter(path);
    let iter = utils::Split::new(finder).filter(|cap| cap.trim().len() > 0);
    iter
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
    use lazy_static::lazy_static;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};

    lazy_static! {
        static ref COMPLEX_JSON_NESTED_OBJECT: Value = json!({
            "string": "value",
            "bool": true,
            "null": null,
            "number": 1,
            "object" : {},
            "array": [],
        });
        static ref COMPLEX_JSON_NESTED_ARRAY: Value = json!([
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
        ]);
        static ref COMPLEX_JSON: Value = json!({
            "string": "value",
            "bool": true,
            "null": null,
            "number": 1,
            "0": 1,
            "object": *COMPLEX_JSON_NESTED_OBJECT,
            "array": *COMPLEX_JSON_NESTED_ARRAY
        });
    }

    fn build_expected_tuple<I>(iter: I) -> (Option<Value>, Option<Value>)
    where
        I: IntoIterator,
        I::Item: Into<Option<Value>> + Clone,
    {
        let expected: Vec<_> = iter.into_iter().map(Into::into).collect();
        let expected: Vec<_> = expected.into_iter().cycle().take(2).collect();
        (expected[0].clone(), expected[1].clone())
    }

    macro_rules! get_index_tests {
        ($name:ident: $val:ident { $($path:literal: $index:expr => $expected:expr,)* }) => {
            #[test]
            fn $name() {
                let mut value = $val.clone();
                $(
                    let (path, index) = build_expected_tuple($expected);
                    let found = (
                        value.get_path($path),
                        value.get_index($index)
                    );
                    assert_eq!(
                        found,
                        (path.as_ref(), index.as_ref()),
                        "(get_path({}), get_index({}))", $path, $path,
                    );
                )*
            }

            paste::item! {
                #[test]
                pub fn [< $name _ mut >]() {
                    let mut value = $val.clone();
                    $(
                        let expected = build_expected_tuple($expected);
                        let found = (
                            value.get_path_mut($path).cloned(),
                            value.get_index_mut($index).cloned(),
                        );
                        assert_eq!(
                            found,
                            expected,
                            "(get_path({}), get_index({}))", $path, $path,
                        );
                    )*
                }
            }
        }
    }

    get_index_tests!(test_complex_json_get_index: COMPLEX_JSON{
        "/string": index!("string") => [json!("value")],
        "/bool": index!("bool") => [json!(true)],
        "bool": index!("bool") => [json!(true)],
        "/null": index!("null") => [json!(null)],
        "/number": index!("number") => [json!(1)],
        "/0": index!("0") => [json!(1)],
        "/0": index!(0) => [Some(json!(1)), None],
        "/object": index!("object") => [COMPLEX_JSON_NESTED_OBJECT.clone()],
        "/object/string": index!("object", "string") => [json!("value")],
        "/object/bool": index!("object", "bool") => [json!(true)],
        "/object/array": index!("object", "array") => [json!([])],
        "/object/array/0": index!("object", "array", 0) => [None],
        "/object/object": index!("object", "object") => [json!({})],
        "/object/object/empty": index!("object", "object", "empty") => [None],
        "/object/object/0": index!("object", "object", 0) => [None],
        "/object/0": index!("object", 0) => [None],
        "/array": index!("array") => [COMPLEX_JSON_NESTED_ARRAY.clone()],
        "/array/0": index!("array", 0) => [json!("value")],
        "/array/1": index!("array", 1) => [json!(true)],
        "/array/'1'": index!("array", "1") => [None],
        "/array/2": index!("array", 2) => [json!(null)],
        "/array/100": index!("array", 100) => [None],
    });

    #[test]
    fn test_split_path_regex() {
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
    fn test_index_value_by_path() {
        let value = json!({
            "1": 1,
            "2": { "hello": "world" },
            "3": [ true, "hello", 3 ],
        });

        assert_eq!(value[index!("1")], json!(1));
        assert_eq!(value[index!("2")], json!({ "hello": "world" }));
        assert_eq!(value[index!("2", "hello")], json!("world"));
        assert_eq!(value[index!("2", "missing")], json!(null));
        assert_eq!(value[index!("3")], json!([true, "hello", 3]));
        assert_eq!(value[index!("3", 0)], json!(true));
        assert_eq!(value[index!("3", "0")], json!(null));
        assert_eq!(value[index!("3", 1)], json!("hello"));
        assert_eq!(value[index!("3", 2)], json!(3));
        let array = &value[index!("3")];
        assert_eq!(array[0], json!(true));
        assert_eq!(array[1], json!("hello"));
        assert_eq!(array[5], json!(null));
    }

    #[test]
    fn test_index_mut_value_by_path() {
        let mut value = json!({
            "1": 1,
            "2": { "hello": "world" },
        });

        // change existing nested value
        value[index!("2", "hello")] = json!("world 2");
        assert_eq!(
            value["2"]["hello"],
            json!("world 2"),
            "change value /'2'/hello"
        );

        // insert new value
        value[index!("3")] = json!(3);
        assert_eq!(value["3"], json!(3), "insert new value /'3'");

        // insert nested value
        value[index!("2", "new")] = json!([1, 2, 3]);
        assert_eq!(
            value["2"]["new"],
            json!([1, 2, 3]),
            "insert new nested value /'2'/new"
        );

        // mutable pointer to null is returned but not inserted
        assert_eq!(value[index!("i did nothing")], json!(null));

        // check the full value
        assert_eq!(
            value,
            json!({
                "1": 1,
                "2": {
                    "hello": "world 2",
                    "new": [1, 2, 3]
                },
                "3": 3,
            })
        );
    }

    #[test]
    fn test_index_path_indexing() {
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
    }

    #[test]
    fn test_index_path_slicing() {
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
        assert_eq!(value.get_index(&index[..3]), Some(&json!([1, 2, 3])));
        assert_eq!(value.get_index(&index[..4]), Some(&json!(2)));
    }

    #[test]
    fn test_get_index_arguments() {
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
    }
}
