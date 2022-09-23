use itertools::Itertools;
use serde_json::{Map, Value};
use std::borrow::Borrow;
use std::rc::Rc;

pub type IndexRef = Rc<dyn serde_json::value::Index>;

#[derive(Clone)]
struct PathIter(std::vec::IntoIter<IndexRef>);
// pub struct PathIter<Iter>(Iter);
// where
//     Iter: IntoIterator,
//     Iter::Item: Borrow<IndexRef>;

// impl Iterator for PathIter {
//     type Item = IndexRef;

//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

impl IntoIterator for PathIter {
    type Item = IndexRef;
    // type Item = Iter::Item; // IndexRef;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    // type IntoIter = Iter::IntoIter; // std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0 // .into_iter()
    }
}

// impl<Iter> Iterator for PathIter<Iter> {
//     // We can refer to this type using Self::Item
//     type Item = IndexRef;

//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }

// impl<Iter> IntoIterator for PathIter<Iter>
// where
//     Iter: IntoIterator,
//     Iter::Item: Borrow<IndexRef>,
// {
//     type Item = IndexRef;
//     // type Item = Iter::Item; // IndexRef;
//     type IntoIter = std::vec::IntoIter<Self::Item>;
//     // type IntoIter = Iter::IntoIter; // std::vec::IntoIter<Self::Item>;

//     fn into_iter(self) -> Self::IntoIter {
//         self.0.into_iter()
//     }
// }

// impl IntoIterator for Path {
//     type Item = IndexRef;
//     type IntoIter = PathIter; // <std::vec::IntoIter<Self::Item>>;

//     fn into_iter(self) -> Self::IntoIter {
//         PathIter(self.0.into_iter())
//     }
// }

#[derive(Clone, Default)]
pub struct Path(Vec<IndexRef>);

impl Path {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, index: impl serde_json::value::Index + 'static) {
        self.0.push(Rc::new(index));
    }
}

// // impl<I> std::ops::Index<usize> for Path
// impl<'a> std::ops::Index<usize> for &'a Path
// // where
// //     I: serde_json::value::Index,
// {
//     type Output = &'a dyn serde_json::value::Index;

//     fn index(self, index: usize) -> Self::Output {
//         self.0[index].as_ref()
//         // static NULL: Value = Value::Null;
//         // self.get_index(index).unwrap_or(&NULL)
//     }
// }

// impl<T> std::ops::Index<T> for Value
// where
//     T: AsRef<dyn serde_json::value::Index>,
//     // I: serde_json::value::Index,
// {
//     type Output = Value;

//     fn index(&self, index: T) -> &Self::Output {
//         static NULL: Value = Value::Null;
//         index.as_ref().index_into(self).unwrap_or(&NULL)
//         // self.get_index(index).unwrap_or(&NULL)
//     }
// }

impl std::ops::Index<Path> for Value {
    type Output = Value;

    fn index(&self, index: Path) -> &Self::Output {
        static NULL: Value = Value::Null;
        self.get_index(index).unwrap_or(&NULL)
    }
}

// impl std::ops::IndexMut<Path> for Value {
//     fn index(&self, index: Path) -> &mut Self::Output {
//         static NULL: Value = Self::Output::Null;
//         // index.as_ref().index_into(self).unwrap_or(&NULL)
//         todo!()
//         // self.get_index(index).unwrap_or(&NULL)
//         // .index_into(self)
//     }
// }

impl std::ops::Index<std::ops::Range<usize>> for Path {
    type Output = Path;

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        Self::from_iter(self.0[index.start..index.end].iter().cloned())
        // &SliceWrapper {slice: self.vec[index]}
    }
}

// pub trait AsRef<IndexRef> for Path {
//     fn as_ref(&self) -> &IndexRef {
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

impl FromIterator<IndexRef> for Path {
    fn from_iter<I: IntoIterator<Item = IndexRef>>(iter: I) -> Self {
        let mut path = Path::new();
        path.extend(iter);
        path
    }
}

pub trait Index<I> {
    // fn get_path<'a>(&'a self, indices: I) -> Option<&'a Value>;
    // fn get_path_mut<'a>(&'a mut self, indices: I) -> Option<&'a mut Value>;

    fn get_index<'a>(&'a self, indices: I) -> Option<&'a Value>;
    fn get_index_mut<'a>(&'a mut self, indices: I) -> Option<&'a mut Value>;
}

// impl<Iter> Index<PathIter<Iter>> for Value
// impl Index<PathIter> for Value
impl Index<Path> for Value
// where
//     Iter: IntoIterator,
//     Iter::Item: Borrow<IndexRef>,
{
    // fn get_index<'a>(&'a self, indices: PathIter<Iter>) -> Option<&'a Value> {
    fn get_index<'a>(&'a self, indices: Path) -> Option<&'a Value> {
        let mut val: Option<&'a Value> = Some(self);
        for index in indices.iter() {
            val = match val {
                Some(v) => v.get(index.as_ref()),
                None => return None,
            };
        }
        val
    }

    fn get_index_mut<'a>(&'a mut self, indices: Path) -> Option<&'a mut Value> {
        let mut val: Option<&'a mut Value> = Some(self);
        for index in indices.iter() {
            val = match val {
                Some(v) => v.get_mut(index.as_ref()),
                None => return None,
            };
        }
        val
    }
}

impl<I> Index<I> for Value
where
    I: IntoIterator,
    I::Item: Borrow<str>,
{
    fn get_index<'a>(&'a self, indices: I) -> Option<&'a Value> {
        let mut val: Option<&'a Value> = Some(self);
        for str_index in indices.into_iter() {
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

    fn get_index_mut<'a>(&'a mut self, indices: I) -> Option<&'a mut Value> {
        let mut val: Option<&'a mut Value> = Some(self);
        for str_index in indices.into_iter() {
            let str_index = str_index.borrow();
            match val {
                Some(Value::Array(_)) => {
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
    use crate::test::assert_matches;
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
        ($name:ident: $val:ident { $($help:literal: $path:expr => $expected:expr,)* }) => {
            #[test]
            fn $name() -> Result<()> {
                let mut value = $val.clone();
                $(
                    let expected = Option::from($expected);
                    assert_eq!(value.get_index($path), expected.as_ref());
                )*
                Ok(())
            }

            paste::item! {
                #[test]
                pub fn [< $name _ mut >]() -> Result<()> {
                    let mut value = $val.clone();
                    $(
                        let mut expected = Option::from($expected);
                        assert_eq!(value.get_index_mut($path), expected.as_mut());
                    )*
                    Ok(())
                }
            }
        }
    }

    get_index_tests!(test_complex_json_get_index: COMPLEX_JSON{
        "test": index!("string") => Value::String("value".into()),
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
        let index = index!("1", "2", "3");
        assert_eq!(
            value.get_index(&index[..1]), // .map(|obj| obj.keys()),
            Some(1)
        );
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
        value.get_index(["string"]);
        value.get_index(["string".to_string()]);
        value.get_index(vec!["string"]);
        value.get_index(vec!["string", "0", "3"]);
        value.get_index(vec!["string".to_string()]);
        value.get_index(std::collections::VecDeque::from_iter(["string"]));

        let test = vec!["test"];
        value.get_index(test.into_iter());
        Ok(())
    }
}
