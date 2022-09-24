use super::utils;
use fancy_regex::Regex;
use itertools::Itertools;
use serde_json::{Map, Value};
use std::any::Any;
use std::borrow::Borrow;
use std::rc::Rc;

// pub trait JsonIndex: serde_json::value::Index + PartialEq<Rhs=usize> {}
// pub trait JsonIndex: serde_json::value::Index + ToString + PartialEq {}
pub trait JsonIndex: serde_json::value::Index + std::fmt::Display + std::fmt::Debug {
    // fn eq(&self, other: &dyn JsonIndex) -> bool;
    fn eq(&self, other: &dyn Test) -> bool;
    // fn as_any(&self) -> Box<dyn Any>; // &dyn Any;
    // fn as_any<'a>(&'a self) -> &'a (dyn Any + 'a);
    // fn as_any(&self) -> &dyn Any;
    // fn as_usize(&self) -> Option<usize>;
    // fn as_str(&self) -> Option<&str>;
}

// pub trait JsonIndexAny {
//     fn eq(&self, other: &dyn JsonIndex) -> bool;
//     fn as_any(&self) -> &dyn Any;
// }

impl JsonIndex for str
// where
//     Self: 'static,
{
    // fn eq(&self, other: &dyn JsonIndex) -> bool {
    fn eq(&self, other: &dyn Test) -> bool {
        false
    }
    // // fn as_any(&self) -> Box<dyn Any + '_> { // &dyn Any { // Box<dyn Any> {
    // // fn as_any<'a>(&'a self) -> &'a (dyn Any + 'a) {
    // fn as_any(&self) -> &dyn Any {
    //     &Rc::new(self)
    //     // &&self[..]
    //     // Box::new(&*self)
    // }
    // fn as_usize(&self) -> Option<usize> {
    //     None
    // }
    // fn as_str(&self) -> Option<&str> {
    //     None
    // }
}

impl JsonIndex for String {
    // fn eq(&self, other: &dyn JsonIndex) -> bool {
    fn eq(&self, other: &dyn Test) -> bool {
        false
    }

    // // fn as_any(&self) -> Box<dyn Any> { // &dyn Any {
    // // fn as_any(&self) -> Box<dyn Any> { // &dyn Any {
    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
    // fn as_usize(&self) -> Option<usize> {
    //     None
    // }
    // fn as_str(&self) -> Option<&str> {
    //     None
    // }
}

impl JsonIndex for usize {
    // fn eq(&self, other: &dyn JsonIndex) -> bool {
    fn eq(&self, other: &dyn Test) -> bool {
        // good thing: other implements as_any and we can downcast into our impl
        println!("we are inside the usize");
        dbg!(self);
        dbg!(other.as_any().downcast_ref::<Rc<usize>>());
        dbg!(other.as_any().downcast_ref::<usize>());
        if let Some(other) = other.as_any().downcast_ref::<Rc<usize>>() {
            dbg!(self);
            dbg!(other);
            return true;
        }

        // if let Some(other) = (other as &dyn Any).downcast_ref::<usize>() {
        //     return true;
        // }
        // let other.down
        false
    }

    // // fn as_any(&self) -> Box<dyn Any> { // &dyn Any {
    // fn as_any(&self) -> &dyn Any {
    //     // Box::new(*self)
    //     self
    // }
    // fn as_usize(&self) -> Option<usize> {
    //     Some(*self)
    // }
    // fn as_str(&self) -> Option<&str> {
    //     None
    // }
}

impl<'a, I> JsonIndex for &'a I
where
    I: ?Sized + JsonIndex + Any,
{
    // fn eq(&self, other: &dyn JsonIndex) -> bool {
    fn eq(&self, other: &dyn Test) -> bool {
        false
    }

    // fn as_any(&self) -> &dyn Any {
    //     self
    // }
}

pub trait Test {
    fn eq(&self, other: &dyn Test) -> bool;

    fn as_any(&self) -> &dyn Any;
}

impl<I> Test for Rc<I>
where
    I: ?Sized + JsonIndex + Any,
{
    fn eq(&self, other: &dyn Test) -> bool {
        dbg!(self);
        dbg!(other.as_any().downcast_ref::<Rc<I>>());
        dbg!(other.as_any().downcast_ref::<Rc<usize>>());
        I::eq(self, other);
        // if let Some(other) = other.as_any().downcast_ref::<Rc<I>>() {
        //     dbg!(self);
        //     dbg!(other);
        //     return true;
        // }
        false
    }

    fn as_any(&self) -> &dyn Any {
        self // .as_ref()
    }
}

// impl<'a> Test for &'a str {
//     // fn eq(&self, other: &dyn Test) -> bool {
//     //     false
//     // }

//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// impl Test for str {
//     // fn eq(&self, other: &dyn Test) -> bool {
//     //     false
//     // }

//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// impl Test for String {
//     // fn eq(&self, other: &dyn Test) -> bool {
//     //     false
//     // }

//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// impl Test for &'static str {
//     // fn eq(&self, other: &dyn Test) -> bool {
//     //     false
//     // }

//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// impl<'a, I> Test for &'a I
// where
//     I: ?Sized + JsonIndex,
// {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
// }

// impl<'a, I> JsonIndexAny for &'a I
// where
//     I: ?Sized + JsonIndex + 'static,
// {
//     fn eq(&self, other: &dyn JsonIndex) -> bool {
//         false
//     }

//     // fn as_any(&self) -> Box<dyn Any> { // &dyn Any {
//     fn as_any(&self) -> &dyn Any {
//         self
//     }
//     // fn as_usize(&self) -> Option<usize> {
//     //     (*self).as_usize()
//     // }
//     // fn as_str(&self) -> Option<&str> {
//     //     None
//     // }
// }

// impl<'a> PartialEq<&'a dyn Test> for &'a dyn Test {
//     fn eq(&self, other: &&dyn Test) -> bool {
//         // JsonIndex::eq(self, other)
//         // if let Some(other) = (&other as &dyn Any).downcast_ref::<usize>() {
//         // if let Some(other) = (*other).downcast_ref::<usize>() {
//         //     return true;
//         // }
//         Test::eq(*self, *other)
//     }
// }

// impl PartialEq<Rc<dyn JsonIndex>> for Rc<dyn JsonIndex> {
//     fn eq(&self, other: &Rc<dyn JsonIndex>) -> bool {
impl PartialEq<Path> for Path {
    fn eq(&self, other: &Path) -> bool {
        // JsonIndex::eq(self, other)
        // if let Some(other) = (*other).downcast_ref::<usize>() {
        // if let Some(other) = (&*other as &dyn Any).downcast_ref::<usize>() {
        //     return true;
        // }
        // self and other are &&dyn
        // let test = "test";
        for (i, o) in self.iter().zip(other) {
            // if
            dbg!(i, o);
            if !Test::eq(i, o) {
                return false;
            }
        }
        // Test::eq(*self, *other)
        // JsonIndex::eq(*self, *other)
        false
    }
}

// impl<'a> PartialEq<&'a dyn JsonIndex> for &'a dyn JsonIndex {
//     fn eq(&self, other: &&dyn JsonIndex) -> bool {
//         // JsonIndex::eq(self, other)
//         // if let Some(other) = (*other).downcast_ref::<usize>() {
//         // if let Some(other) = (&*other as &dyn Any).downcast_ref::<usize>() {
//         //     return true;
//         // }
//         // self and other are &&dyn
//         JsonIndex::eq(*self, *other)
//     }
// }

pub type IndexRef = Rc<dyn JsonIndex>;
// pub type IndexRef = Rc<dyn serde_json::value::Index>;

// #[derive(PartialEq, Clone, Default)]
#[derive(Clone, Default)]
pub struct Path(Vec<IndexRef>);

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(ToString::to_string).join("/"))
    }
}

impl std::fmt::Debug for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl IntoIterator for Path {
    type Item = IndexRef;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Path {
    type Item = &'a IndexRef;
    type IntoIter = std::slice::Iter<'a, IndexRef>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl Path {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn iter<'a>(&'a self) -> std::slice::Iter<'a, IndexRef> {
        self.0.iter()
    }

    #[inline]
    // pub fn push(&mut self, index: impl serde_json::value::Index + 'static) {
    pub fn push(&mut self, index: impl JsonIndex + 'static) {
        self.0.push(Rc::new(index));
    }
}

impl std::ops::Deref for Path {
    type Target = Vec<IndexRef>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for Path {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl std::ops::Index<&Path> for Value {
    type Output = Value;

    #[inline]
    fn index(&self, index_path: &Path) -> &Self::Output {
        static NULL: Value = Value::Null;
        self.get_index(index_path).unwrap_or(&NULL)
    }
}

impl std::ops::Index<Path> for Value {
    type Output = Value;

    #[inline]
    fn index(&self, index_path: Path) -> &Self::Output {
        self.index(&index_path)
    }
}

impl std::ops::IndexMut<&Path> for Value {
    #[inline]
    fn index_mut<'a>(&'a mut self, index_path: &Path) -> &'a mut Self::Output {
        let mut val: &'a mut Value = self;
        for index in index_path.into_iter() {
            val = index.as_ref().index_or_insert(val);
        }
        val
    }
}

impl std::ops::IndexMut<Path> for Value {
    #[inline]
    fn index_mut<'a>(&'a mut self, index_path: Path) -> &'a mut Self::Output {
        self.index_mut(&index_path)
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
    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn get_path<'a, P>(&'a self, path: P) -> Option<&'a Value>
    where
        P: Borrow<str>,
    {
        self.get_path_iter(split_path(path.borrow()))
    }

    #[inline]
    fn get_path_mut<'a, P>(&'a mut self, path: P) -> Option<&'a mut Value>
    where
        P: Borrow<str>,
    {
        self.get_path_iter_mut(split_path(path.borrow()))
    }
}

#[inline]
pub fn split_path<'b>(path: &'b str) -> impl Iterator<Item = &'b str> + 'b {
    let finder = SPLIT_PATH_REGEX.find_iter(path);
    let iter = utils::Split::new(finder).filter(|cap| cap.trim().len() > 0);
    iter
}

#[inline]
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

    #[inline]
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

    #[test]
    fn test_json_index_to_string() {
        assert_eq!((&12usize as &dyn JsonIndex).to_string(), "12");
        assert_eq!((&0usize as &dyn JsonIndex).to_string(), "0");
        assert_eq!((&100_000usize as &dyn JsonIndex).to_string(), "100000");
        assert_eq!((&"test" as &dyn JsonIndex).to_string(), "test");
        assert_eq!((&"'test'" as &dyn JsonIndex).to_string(), "'test'");
        assert_eq!((&r#""test""# as &dyn JsonIndex).to_string(), r#""test""#);
        assert_eq!(
            (&String::from("test") as &dyn JsonIndex).to_string(),
            "test"
        );
        assert_eq!(
            (&String::from("'test'") as &dyn JsonIndex).to_string(),
            "'test'"
        );
        assert_eq!(
            (&String::from(r#""test""#) as &dyn JsonIndex).to_string(),
            r#""test""#
        );
    }

    #[test]
    fn test_index_path_to_string() {
        assert_eq!(index!().to_string(), "");
        assert_eq!(index!("test").to_string(), "test");
        assert_eq!(index!("test/hello").to_string(), "test/hello");
        assert_eq!(index!("test", 12, "hi").to_string(), "test/12/hi");
        assert_eq!(index!(12, 0, 42, "hi").to_string(), "12/0/42/hi");
    }

    #[test]
    fn test_json_index_partial_eq() {
        use std::rc::Rc;
        let test: &str = &"hello"[..];
        let test: &str = &test[..];
        // let index: Rc<dyn JsonIndex> = Rc::new(12usize);
        // assert_eq!(index, index);
        assert_eq!(index!(12), index!(24));
        // assert_eq!(&12usize as &dyn JsonIndex, &24usize as &dyn JsonIndex);
    }
}
