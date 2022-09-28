use super::utils;
use fancy_regex::Regex;
use serde_json::Value;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[derive(Hash, PartialOrd, Ord, PartialEq, Eq, Debug, Clone, Copy)]
pub enum Kind<'a> {
    ObjectKey(&'a str),
    ArrayIndex(&'a usize),
}

#[allow(clippy::module_name_repetitions)]
pub trait JsonIndex: serde_json::value::Index + std::fmt::Display + std::fmt::Debug + Send {
    fn kind(&self) -> Kind;

    #[inline]
    fn eq(&self, other: &dyn JsonIndex) -> bool {
        PartialEq::eq(&self.kind(), &other.kind())
    }

    #[inline]
    fn partial_cmp(&self, other: &dyn JsonIndex) -> Option<Ordering> {
        PartialOrd::partial_cmp(&self.kind(), &other.kind())
    }

    #[inline]
    fn cmp(&self, other: &dyn JsonIndex) -> Ordering {
        Ord::cmp(&self.kind(), &other.kind())
    }

    #[inline]
    fn try_as_object_key(&self) -> Option<&str> {
        match self.kind() {
            Kind::ObjectKey(key) => Some(key),
            Kind::ArrayIndex(_) => None,
        }
    }

    #[inline]
    fn try_as_array_index(&self) -> Option<&usize> {
        match self.kind() {
            Kind::ArrayIndex(idx) => Some(idx),
            Kind::ObjectKey(_) => None,
        }
    }

    #[inline]
    fn is_array_index(&self) -> bool {
        self.try_as_array_index().is_some()
    }

    #[inline]
    fn is_object_key(&self) -> bool {
        self.try_as_object_key().is_some()
    }

    #[inline]
    fn try_into_object_key(&self) -> Option<String> {
        self.try_as_object_key().map(ToOwned::to_owned)
    }

    #[inline]
    fn try_into_array_index(&self) -> Option<usize> {
        self.try_as_array_index().map(ToOwned::to_owned)
    }
}

impl JsonIndex for str {
    #[inline]
    fn kind(&self) -> Kind {
        Kind::ObjectKey(self)
    }
}

impl JsonIndex for String {
    #[inline]
    fn kind(&self) -> Kind {
        Kind::ObjectKey(self.as_str())
    }
}

impl JsonIndex for usize {
    #[inline]
    fn kind(&self) -> Kind {
        Kind::ArrayIndex(self)
    }
}

impl<'a, I, O> JsonIndex for &'a I
where
    I: ?Sized + JsonIndex + ToOwned<Owned = O> + Sync,
    O: JsonIndex + Sized,
{
    #[inline]
    fn kind(&self) -> Kind {
        JsonIndex::kind(*self)
    }
}

impl Hash for dyn JsonIndex + '_ {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.kind().hash(state);
    }
}

impl PartialEq for dyn JsonIndex + '_ {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        JsonIndex::eq(self, other)
    }
}

impl Eq for dyn JsonIndex + '_ {}

impl PartialOrd for dyn JsonIndex + '_ {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        JsonIndex::partial_cmp(self, other)
    }
}

impl Ord for dyn JsonIndex + '_ {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        JsonIndex::cmp(self, other)
    }
}

#[allow(clippy::module_name_repetitions)]
pub type IndexRef = Rc<dyn JsonIndex>;

#[derive(Hash, PartialEq, Eq, Ord, PartialOrd, Clone, Default)]
pub struct Path(Vec<IndexRef>);

impl std::fmt::Display for Path {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "/{}",
            self.0
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join("/")
        )
    }
}

impl std::fmt::Debug for Path {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T> From<T> for Path
where
    T: JsonIndex + 'static,
{
    #[inline]
    fn from(path: T) -> Self {
        Self::new(path)
    }
}

impl FromIterator<IndexRef> for Path {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = IndexRef>,
    {
        let mut path = Self::empty();
        path.extend(iter);
        path
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
    #[must_use]
    pub fn empty() -> Self {
        Self::default()
    }

    #[inline]
    #[must_use]
    pub fn join(mut self, other: &Path) -> Self {
        self.extend(other.iter().cloned());
        self
    }

    #[inline]
    #[must_use]
    pub fn new(index: impl JsonIndex + 'static) -> Self {
        let index: Rc<dyn JsonIndex> = Rc::new(index);
        Self::from_iter([index])
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<IndexRef> {
        self.0.iter()
    }

    #[inline]
    pub fn add(&mut self, index: impl JsonIndex + 'static) {
        self.0.push(Rc::new(index));
    }

    #[inline]
    #[must_use]
    pub fn depth(&self) -> usize {
        self.0.len()
    }

    #[inline]
    #[must_use]
    pub fn is_object_key(&self) -> bool {
        self.0.last().map_or(false, |idx| idx.is_object_key())
    }

    #[inline]
    #[must_use]
    pub fn is_array_key(&self) -> bool {
        self.0.last().map_or(false, |idx| idx.is_array_index())
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
    fn index_mut<'a>(&'a mut self, path: &Path) -> &'a mut Self::Output {
        let mut val: &'a mut Value = self;
        for index in path {
            val = index.as_ref().index_or_insert(val);
        }
        val
    }
}

impl std::ops::IndexMut<Path> for Value {
    #[inline]
    fn index_mut(&mut self, index_path: Path) -> &mut Self::Output {
        self.index_mut(&index_path)
    }
}

pub trait Index {
    fn get_path<S>(&self, path: S) -> Option<&Value>
    where
        S: Borrow<str>;

    fn get_path_mut<S>(&mut self, path: S) -> Option<&mut Value>
    where
        S: Borrow<str>;

    fn get_path_iter<P>(&self, path_iter: P) -> Option<&Value>
    where
        P: IntoIterator,
        P::Item: Borrow<str>;

    fn get_path_iter_mut<P>(&mut self, path_iter: P) -> Option<&mut Value>
    where
        P: IntoIterator,
        P::Item: Borrow<str>;

    fn get_index<I>(&self, indices: I) -> Option<&Value>
    where
        I: IntoIterator,
        I::Item: Borrow<IndexRef>;

    fn get_index_mut<I>(&mut self, indices: I) -> Option<&mut Value>
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
        for index in indices {
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
        for index in indices {
            val = match val {
                Some(v) => v.get_mut(index.borrow().as_ref()),
                None => return None,
            };
        }
        val
    }

    #[inline]
    fn get_path_iter<'a, P>(&'a self, path: P) -> Option<&'a Value>
    where
        P: IntoIterator,
        P::Item: Borrow<str>,
    {
        let mut val: Option<&'a Value> = Some(self);
        for str_index in path {
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
    fn get_path_iter_mut<'a, P>(&'a mut self, path: P) -> Option<&'a mut Value>
    where
        P: IntoIterator,
        P::Item: Borrow<str>,
    {
        let mut val: Option<&'a mut Value> = Some(self);
        for str_index in path {
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
    fn get_path<P>(&self, path: P) -> Option<&Value>
    where
        P: Borrow<str>,
    {
        self.get_path_iter(split_path(path.borrow()))
    }

    #[inline]
    fn get_path_mut<P>(&mut self, path: P) -> Option<&mut Value>
    where
        P: Borrow<str>,
    {
        self.get_path_iter_mut(split_path(path.borrow()))
    }
}

#[inline]
pub fn split_path(path: &str) -> impl Iterator<Item = &str> {
    let finder = SPLIT_PATH_REGEX.find_iter(path);
    let iter = utils::Split::new(finder).filter(|cap| !cap.trim().is_empty());
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

#[macro_export]
macro_rules! index {
    ( $( $idx:expr ),* ) => {{
        #[allow(unused_mut)]
        let mut index = $crate::index::Path::empty();
        $(
            index.add($idx);
        )*
        index
    }};
}

#[cfg(test)]
pub mod test {
    use super::{is_integer, split_path, Index, Kind};
    use crate::test::ValueExt;
    use lazy_static::lazy_static;
    use pretty_assertions::assert_eq;
    use serde_json::{json, Value};
    use std::collections::VecDeque;

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
        I::IntoIter: Clone,
        I::Item: Into<Option<Value>> + Clone,
    {
        let expected: Vec<Option<Value>> =
            iter.into_iter().map(Into::into).cycle().take(2).collect();
        (expected[0].clone(), expected[1].clone())
    }

    macro_rules! idx {
        ( $idx:expr ) => {{
            let test: std::rc::Rc<dyn $crate::index::JsonIndex> = std::rc::Rc::new($idx);
            test
        }};
    }

    macro_rules! get_index_tests {
        ($name:ident: $val:ident { $($path:literal: $index:expr => $expected:expr,)* }) => {
            #[test]
            fn $name() {
                let value = $val.clone();
                $(
                    let (path, index) = build_expected_tuple($expected);
                    let found = (
                        value.get_path($path),
                        value.get_index($index)
                    );
                    assert_eq!(
                        &found,
                        &(path.as_ref(), index.as_ref()),
                        "(get_path({}), get_index({}))", $path, $index,
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
                            &found,
                            &expected,
                            "(get_path({}), get_index({}))", $path, $index,
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
        assert_eq!(&split_path("test").collect::<Vec<&str>>(), &vec!["test"]);
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
            &value["2"]["hello"],
            &json!("world 2"),
            "change value /'2'/hello"
        );

        // insert new value
        value[index!("3")] = json!(3);
        assert_eq!(&value["3"], &json!(3), "insert new value /'3'");

        // insert nested value
        value[index!("2", "new")] = json!([1, 2, 3]);
        assert_eq!(
            &value["2"]["new"],
            &json!([1, 2, 3]),
            "insert new nested value /'2'/new"
        );

        // mutable pointer to null is returned but not inserted
        assert_eq!(&value[index!("i did nothing")], &json!(null));

        // check the full value
        assert_eq!(
            &value,
            &json!({
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
            &value.get_index(&index[..0]).try_keys(),
            &Some(vec!["1".into()])
        );
        assert_eq!(
            &value.get_index(&index[..1]).try_keys(),
            &Some(vec!["2".into()])
        );
        assert_eq!(
            &value.get_index(&index[..2]).try_keys(),
            &Some(vec!["3".into()])
        );
        assert_eq!(&value.get_index(&index[..3]), &Some(&json!([1, 2, 3])));
        assert_eq!(&value.get_index(&index[..4]), &Some(&json!(2)));
    }

    #[test]
    fn test_get_index_arguments() {
        let value = COMPLEX_JSON.clone();
        value.get_index(index!("string"));
        value.get_index(index!("string".to_string()));
        value.get_index(index!("string", 0));
        value.get_index(index!("string".to_string(), 0, "test"));
    }

    #[test]
    fn test_get_path_iter_arguments() {
        let value = COMPLEX_JSON.clone();
        value.get_path_iter(["string"]);
        value.get_path_iter(["string".to_string()]);
        value.get_path_iter(vec!["string"]);
        value.get_path_iter(vec!["string", "0", "3"]);
        value.get_path_iter(vec!["string".to_string()]);
        value.get_path_iter(VecDeque::from_iter(["string"]));
        let test = vec!["test"];
        value.get_path_iter(test.into_iter());
    }

    #[test]
    fn test_index_to_string() {
        assert_eq!(idx!(12usize).to_string(), "12");
        assert_eq!(idx!(0usize).to_string(), "0");
        assert_eq!(idx!(100_000usize).to_string(), "100000");
        assert_eq!(idx!("test").to_string(), "test");
        assert_eq!(idx!("'test'").to_string(), "'test'");
        assert_eq!(idx!(r#""test""#).to_string(), r#""test""#);
        assert_eq!(idx!(String::from("test")).to_string(), "test");
        assert_eq!(idx!(String::from("'test'")).to_string(), "'test'");
        assert_eq!(idx!(String::from(r#""test""#)).to_string(), r#""test""#);
    }

    #[test]
    fn test_index_path_to_string() {
        assert_eq!(index!().to_string(), "/");
        assert_eq!(index!("test").to_string(), "/test");
        assert_eq!(index!("test/hello").to_string(), "/test/hello");
        assert_eq!(index!("test", 12, "hi").to_string(), "/test/12/hi");
        assert_eq!(index!(12, 0, 42, "hi").to_string(), "/12/0/42/hi");
    }

    #[test]
    fn test_index_kind() {
        assert_eq!(idx!(12usize).kind(), Kind::ArrayIndex(&12usize));
        let usize_ref = 12usize;
        assert_eq!(idx!(&usize_ref).kind(), Kind::ArrayIndex(&12usize));
        assert_eq!(idx!(String::from("test")).kind(), Kind::ObjectKey("test"));
        assert_eq!(idx!("test").kind(), Kind::ObjectKey("test"));
        let string_ref = String::from("test");
        assert_eq!(idx!(&string_ref).kind(), Kind::ObjectKey("test"));
    }

    #[test]
    fn test_index_try_as_array_index() {
        assert_eq!(idx!(12usize).try_as_array_index(), Some(&12usize));
        assert_eq!(idx!("test").try_as_array_index(), None);
        assert_eq!(idx!(String::from("test")).try_as_array_index(), None);
        let string_ref = String::from("test");
        assert_eq!(idx!(&string_ref).try_as_array_index(), None);
    }

    #[test]
    fn test_index_try_as_object_key() {
        assert_eq!(idx!(12usize).try_as_object_key(), None);
        assert_eq!(idx!("test").try_as_object_key(), Some("test"));
        assert_eq!(idx!(String::from("test")).try_as_object_key(), Some("test"));
        let string_ref = String::from("test");
        assert_eq!(idx!(&string_ref).try_as_array_index(), None);
    }

    #[test]
    fn test_index_partial_eq() {
        assert_ne!(&idx!(12usize), &idx!(24usize));
        assert_eq!(&idx!(12usize), &idx!(12usize));
        assert_eq!(&idx!("test"), &idx!("test"));
        assert_ne!(&idx!("test hello"), &idx!("test"));
        assert_eq!(&idx!(String::from("test")), &idx!("test"));
        assert_eq!(&idx!(String::from("test")), &idx!(String::from("test")));

        let s1: String = "test".into();
        let s2: String = "test".into();
        assert_eq!(&idx!(s1.clone()), &idx!(s2.clone()));
        assert_eq!(&idx!(&s1), &idx!(s2.clone()));
        assert_eq!(&idx!(&s1), &idx!(&s2));
        assert_eq!(&idx!(s1), &idx!(&s2));

        let s1: String = "test".into();
        let s2: &str = "test";
        assert_eq!(&idx!(s1.clone()), &idx!(s2));
        assert_eq!(&idx!(&s1), &idx!(s2));
        assert_eq!(&idx!(&s1), &idx!(&s2));
        assert_eq!(&idx!(s1), &idx!(&s2));

        let i1: usize = 100;
        let i2: usize = 100;
        assert_eq!(&idx!(i1), &idx!(i2));
        assert_eq!(&idx!(&i1), &idx!(i2));
        assert_eq!(&idx!(i1), &idx!(&i2));
        assert_eq!(&idx!(&i1), &idx!(&i2));

        let i1: usize = 100;
        let i2: usize = 120;
        assert_ne!(&idx!(i1), &idx!(i2));
        assert_ne!(&idx!(&i1), &idx!(i2));
        assert_ne!(&idx!(i1), &idx!(&i2));
        assert_ne!(&idx!(&i1), &idx!(&i2));

        let s1: &str = "test";
        let s2: &str = "test";
        assert_eq!(&idx!(s1), &idx!(s2));
        assert_eq!(&idx!(&s1), &idx!(s2));
        assert_eq!(&idx!(s1), &idx!(&s2));
        assert_eq!(&idx!(&s1), &idx!(&s2));

        let s1: &str = "test";
        let s2: &str = "different";
        assert_ne!(&idx!(s1), &idx!(s2));
        assert_ne!(&idx!(&s1), &idx!(s2));
        assert_ne!(&idx!(s1), &idx!(&s2));
        assert_ne!(&idx!(&s1), &idx!(&s2));
    }

    #[test]
    fn test_index_partial_ord() {
        use std::cmp::Ordering;
        assert_eq!(&idx!("a").partial_cmp(&idx!("a")), &Some(Ordering::Equal));
        assert_eq!(&idx!("a").partial_cmp(&idx!("b")), &Some(Ordering::Less));
        assert_eq!(
            idx!("a").partial_cmp(&idx!("b".to_string())),
            Some(Ordering::Less)
        );
        assert_eq!(
            idx!("a".to_string()).partial_cmp(&idx!("b".to_string())),
            Some(Ordering::Less)
        );
        assert_eq!(
            idx!(0usize).partial_cmp(&idx!(0usize)),
            Some(Ordering::Equal)
        );
        assert_eq!(
            idx!(0usize).partial_cmp(&idx!(10usize)),
            Some(Ordering::Less)
        );
        assert_eq!(
            idx!(10usize).partial_cmp(&idx!(0usize)),
            Some(Ordering::Greater)
        );
        assert_eq!(&idx!("aa").partial_cmp(&idx!("b")), &Some(Ordering::Less));
        assert_eq!(idx!("bab").partial_cmp(&idx!("b")), Some(Ordering::Greater));
    }

    #[test]
    fn test_index_path_ord() {
        macro_rules! sort_index {
            ( $path:expr ) => {{
                let mut inner = (*$path).clone();
                inner.sort();
                $crate::index::Path::from_iter(inner)
            }};
        }

        assert_eq!(sort_index!(index!(1, 2, 4)), index!(1, 2, 4));
        assert_eq!(sort_index!(index!(5, 1, 4, 3, 4)), index!(1, 3, 4, 4, 5));
        assert_eq!(sort_index!(index!("b", "c", "a")), index!("a", "b", "c"));
        assert_eq!(
            sort_index!(index!("b".to_string(), "c", "a")),
            index!("a", "b", "c")
        );
        assert_eq!(
            sort_index!(index!(4, 2, 3, 1, "b", "c", "a")),
            index!("a", "b", "c", 1, 2, 3, 4)
        );
    }

    #[test]
    fn test_index_path_partial_eq() {
        assert_eq!(index!(), index!());
        assert_ne!(index!(12), index!());
        assert_ne!(index!(12), index!(24));
        assert_ne!(index!("hello", 12), index!("hello", 24));
        assert_eq!(index!("hello", 12), index!("hello", 12));
        assert_eq!(index!("hello", "world"), index!("hello", "world"));
        assert_ne!(index!("hello", "world"), index!("world", "hello"));
        assert_eq!(
            index!(String::from("hello"), "world"),
            index!("hello", "world")
        );
        assert_eq!(
            index!(String::from("hello"), "world"),
            index!(String::from("hello"), "world")
        );
        assert_eq!(
            index!(String::from("hello"), 12, "world", 1),
            index!(String::from("hello"), 12, "world", 1)
        );
        assert_ne!(
            index!(String::from("hello"), 12, "world", 1),
            index!(String::from("hello"), 56, "world", 1)
        );
    }

    #[test]
    fn test_value_str_string_equality() {
        let value = json!({"a": 42});
        let str_index: &str = "a";
        let string_index: String = "a".into();
        assert_eq!(value[str_index], json!(42));
        assert_eq!(value[&str_index], json!(42));
        assert_eq!(value[&string_index], json!(42));
        assert_eq!(value[string_index.clone()], json!(42));
        assert_eq!(value[str_index], value[string_index.clone()]);
        assert_eq!(value[str_index], value[&string_index]);
        assert_eq!(value[&str_index], value[string_index.clone()]);
        assert_eq!(value[&str_index], value[&string_index]);
    }
}
