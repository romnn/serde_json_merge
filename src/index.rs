use itertools::Itertools;
use serde_json::{Map, Value};
use std::borrow::Borrow;
use std::rc::Rc;

pub type IndexRef = Rc<dyn serde_json::value::Index>;

// #[derive(Clone)]
// pub struct IndexPath<I>(I)
// //  {} // Vec<IndexRef>);
// where
//     I: IntoIterator, //  {} // Vec<IndexRef>);
//     I::Item: Borrow<IndexRef>;
//
#[derive(Clone, Default)]
pub struct IndexPath(Vec<IndexRef>);

impl IndexPath {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, index: impl serde_json::value::Index + 'static) {
        self.0.push(Rc::new(index));
    }
}

impl std::ops::Deref for IndexPath {
    type Target = Vec<IndexRef>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for IndexPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl FromIterator<IndexRef> for IndexPath {
    fn from_iter<I: IntoIterator<Item = IndexRef>>(iter: I) -> Self {
        let mut path = IndexPath::new();
        path.extend(iter);
        path
        // for index in iter {
        //     path.add(index);
        // }
        // c
    }
}

// trait SealedIndex<I> // where
trait Index<I> // where
//     Self: Value,
{
    // type Indices;

    fn get_index<'a>(&'a self, indices: I) -> Option<&'a Value>;

    fn get_index_mut<'a>(&'a mut self, indices: I) -> Option<&'a mut Value>;
}

// impl<T> From<T> for IndexPath<T>
// where
//     T: IntoIterator, //  {} // Vec<IndexRef>);
//     T::Item: Borrow<IndexRef>,
// {
//     fn from(other: T) -> Self {
//         Self(other)
//     }
// }

// #[derive(Clone)]
// pub struct StringPath<I>(I)
// where
//     I: IntoIterator, //  {} // Vec<IndexRef>);
//     I::Item: Borrow<String>;

// impl<T> From<T> for StringPath<T>
// where
//     T: IntoIterator,
//     T::Item: Borrow<String>,
// {
//     fn from(other: T) -> Self {
//         Self(other)
//     }
// }
// #[derive(Clone)]
// pub struct StringPath {}; // Vec<IndexRef>);

// pub trait Indices
// where
//     Self: IntoIterator,
//     Self::Item: Borrow<IndexRef>,
// {
//     // fn index() -> T;
// }

// impl<T> Indices for T
// where
//     T: IntoIterator,
//     T::Item: Borrow<IndexRef>,
// {
// }

// pub trait IntoIndex<T>
// where
//     T: IntoIterator,
//     T::Item: Borrow<IndexRef>,
// {
//     fn index() -> T;
// }

// impl<T> IntoIndex<T> for T
// where
//     T: IntoIterator,
//     T::Item: Borrow<IndexRef>,
// {
//     fn index(self) -> T {
//         self
//     }
// }

// impl IntoIndex for Vec<IndexRef> {}

// impl IntoIndex for String {
//     fn index() -> Self {}
// }

// impl IntoIterator for String {
//     type Item = Vec<IndexRef>;
//     // fn into(s: String) -> Vec<IndexRef> {
//     //     vec![]
//     // }
// }

// impl Into<Vec<IndexRef>> for String {
//     fn into(s: String) -> Vec<IndexRef> {
//         vec![]
//     }
// }
// impl Indices for IndexRef {}

// pub trait Index<I>: SealedIndex<I>
// where
//     I: IntoIterator,
//     I::Item: Borrow<IndexRef>,
// {
// }
// pub trait StringIndex: SealedIndex<String> {}

// impl<I> Index<I> for Value
// where
//     I: Iterator,
//     I::Item: Borrow<IndexRef>,
// impl<I> Index<I> for Value
// impl<I> Index<IndexPath<I>> for Value
impl Index<IndexPath> for Value
// impl<I> Index<I> for Value
where
// I: IntoIterator, //  {} // Vec<IndexRef>);
// I::Item: Borrow<IndexRef>,
// where
//     I: Indices,
//     // I: Iterator,
//     // I: Indices,
//     I::Item: Borrow<IndexRef>,
{
    // type Indices = I;

    fn get_index<'a>(&'a self, indices: IndexPath) -> Option<&'a Value> {
        None
        // let mut val: Option<&'a Value> = Some(self);
        // for index in indices.into_iter() {
        //     val = val.and_then(|v| v.get(index.borrow().as_ref()));
        // }
        // val
    }

    fn get_index_mut<'a>(&'a mut self, indices: IndexPath) -> Option<&'a mut Value> {
        None
        // let mut val: Option<&'a mut Value> = Some(self);
        // for index in indices.into_iter() {
        //     val = val.and_then(|v| v.get_mut(index.borrow().as_ref()));
        // }
        // val
    }
}

// trait StringIndex
// where
//     Self: IntoIterator,
//     Self::Item: Borrow<String>,
// {
// }

// impl<I> Index<StringPath<I>> for Value
impl<I> Index<I> for Value
where
    // I: IntoIterator,
    I: IntoIterator,
    // I::Item: AsRef<&'i str>,
    I::Item: Borrow<str>,
    // I::Item: Borrow<String>,
    // impl<S> Index for Value
    // where
    //     S: Borrow<String>, // where
    //     I: Iterator<Item = String>,
    // impl<I> Index<I> for Value
    // where
    //     I: Iterator<Item = String>,
{
    // type Indices = S;
    // I: IntoIterator,
    // I::Item: Borrow<String>,
    // impl StringIndex for Value {
    fn get_index<'a>(&'a self, indices: I) -> Option<&'a Value> {
        None
        // let mut val: Option<&'a Value> = Some(self);
        // for index in indices.into_iter() {
        //     val = val.and_then(|v| v.get(index.borrow().as_ref()));
        // }
        // val
    }

    fn get_index_mut<'a>(&'a mut self, indices: I) -> Option<&'a mut Value> {
        None
        // let mut val: Option<&'a mut Value> = Some(self);
        // for index in indices.into_iter() {
        //     val = val.and_then(|v| v.get_mut(index.borrow().as_ref()));
        // }
        // val
    }
}

// impl Index<String> for Value
// // where
// //     I: Into<String>,
// {
//     fn get_index<'a>(&'a self, indices: String) -> Option<&'a Value> {
//         None
//     }

//     fn get_index_mut<'a>(&'a mut self, indices: String) -> Option<&'a mut Value> {
//         None
//     }
// }

// pub trait Index {
//     fn get_index<'a, I>(&'a self, indices: I) -> Option<&'a Value>
//     where
//         I: IntoIterator,
//         I::Item: Borrow<IndexRef>;

//     fn get_index_mut<'a, I>(&'a mut self, indices: I) -> Option<&'a mut Value>
//     where
//         I: IntoIterator,
//         I::Item: Borrow<IndexRef>;
// }

// impl Index for serde_json::Value {
//     fn get_index<'a, I>(&'a self, indices: I) -> Option<&'a Value>
//     where
//         I: IntoIterator,
//         I::Item: Borrow<IndexRef>,
//     {
//         let mut val: Option<&'a Value> = Some(self);
//         for index in indices.into_iter() {
//             val = val.and_then(|v| v.get(index.borrow().as_ref()));
//         }
//         val
//     }

//     fn get_index_mut<'i, 'a, I>(&'a mut self, indices: I) -> Option<&'a mut Value>
//     where
//         I: IntoIterator,
//         I::Item: Borrow<IndexRef>,
//     {
//         let mut val: Option<&'a mut Value> = Some(self);
//         for index in indices.into_iter() {
//             val = val.and_then(|v| v.get_mut(index.borrow().as_ref()));
//         }
//         val
//     }
// }

macro_rules! index {
    ( $( $idx:expr ),* ) => {{
        // let mut index: Vec<$crate::index::IndexRef> = Vec::new();
        let mut index = $crate::index::IndexPath::new();
        $(
            index.push($idx); // std::rc::Rc::new($idx));
        )*
        index
        // IndexPath::from(index)
    }};
}

#[cfg(test)]
pub mod test {
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
    fn test_get_index_arguments() -> Result<()> {
        let mut value = COMPLEX_JSON.clone();
        value.get_index(index!("string"));
        value.get_index(index!("string".to_string()));
        // this does not work
        let test = "key";
        value.get_index(index!(test));

        value.get_index(["string"]);
        value.get_index(["string".to_string()]);
        value.get_index(vec!["string"]);
        value.get_index(vec!["string".to_string()]);
        value.get_index(std::collections::VecDeque::from_iter(["string"]));

        let test = vec!["test"];
        value.get_index(test.into_iter());
        Ok(())
    }

    #[test]
    fn test_get_index() -> Result<()> {
        let mut value = COMPLEX_JSON.clone();
        assert_eq!(value.get_index(index!("string")), string!("value"),);
        // assert_eq!(value.get_index_mut(index!("string")), mut_string!("value"),);
        assert_eq!(value.get_index(["string"]), string!("value"),);
        Ok(())
    }

    // #[test]
    // fn test_get_index_mut() -> Result<()> {
    //     let mut value = COMPLEX_JSON.clone();
    //     assert_eq!(value.get_index_mut(index!("string")), mut_string!("value"),);
    //     Ok(())
    // }
}
