use crate::index::{IndexRef, Kind as IndexKind, Path as IndexPath};
use crate::iter::{dfs::Dfs, Iter};
use ordered_float::OrderedFloat;
use std::cmp::Ordering;

#[derive(Clone, Default, Debug)]
pub struct Map<'a>(indexmap::IndexMap<String, Value<'a>>);

impl<'a> std::ops::Deref for Map<'a> {
    type Target = indexmap::IndexMap<String, Value<'a>>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> std::ops::DerefMut for Map<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<'a> FromIterator<(String, Value<'a>)> for Map<'a> {
    #[inline]
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (String, Value<'a>)>,
    {
        Self(indexmap::IndexMap::from_iter(iter))
    }
}

#[derive(Clone, Debug, Ord, PartialOrd, PartialEq, Eq)]
pub enum Value<'a> {
    Null,
    Bool(&'a bool),
    Number(OrderedFloat<f64>),
    String(&'a str),
    Array(Vec<Value<'a>>),
    Object(Map<'a>),
}

impl<'a> Value<'a> {
    #[inline]
    pub fn get(&self, index: &IndexRef) -> Option<&Self> {
        match index.kind() {
            IndexKind::ObjectKey(key) => match self {
                Value::Object(map) => map.get(key),
                _ => None,
            },
            IndexKind::ArrayIndex(idx) => match self {
                Value::Array(vec) => vec.get(*idx),
                _ => None,
            },
        }
    }

    #[inline]
    /// Gets or inserts value at `index` and returns a mutable reference to it.
    ///
    /// If the current value is `Value::Null`,
    /// the default value depending on the index type is inserted.
    ///
    /// # Panics
    ///
    /// Panics when
    /// - using an array index to access a value other than `Value::Array`.
    /// - using an array index that is out of bounds
    /// - using an object index to access a value other than `Value::Object`.
    ///
    pub fn get_or_insert(&mut self, index: &IndexRef) -> &mut Self {
        match index.kind() {
            IndexKind::ObjectKey(key) => {
                if let Value::Null = self {
                    *self = Value::Object(Map::default());
                }
                match self {
                    Value::Object(map) => map.entry(key.to_string()).or_insert(Value::Null),
                    _ => panic!("cannot access key {key:?} in {self:?}"),
                }
            }
            IndexKind::ArrayIndex(idx) => {
                if let Value::Null = self {
                    *self = Value::Array(Vec::default());
                }
                match self {
                    Value::Array(vec) => {
                        let len = vec.len();
                        vec.get_mut(*idx).unwrap_or_else(|| {
                            panic!("cannot access index {idx} of array of length {len}")
                        })
                    }
                    _ => panic!("cannot access index {idx} of {self:?}"),
                }
            }
        }
    }
}

static NULL: Value<'static> = Value::Null;

impl<'a> std::ops::Index<&IndexPath> for Value<'a> {
    type Output = Value<'a>;

    #[inline]
    fn index(&self, index_path: &IndexPath) -> &Self::Output {
        let mut val: &Value = self;
        for index in index_path.iter() {
            match val.get(index) {
                Some(new_val) => val = new_val,
                None => return &NULL,
            }
        }
        val
    }
}

impl<'a> std::ops::IndexMut<&IndexPath> for Value<'a> {
    #[inline]
    fn index_mut(&mut self, index_path: &IndexPath) -> &mut Self::Output {
        let mut val: &mut Value = self;
        for index in index_path {
            val = val.get_or_insert(index);
        }
        val
    }
}

impl<'a> From<&'a serde_json::Value> for Value<'a> {
    #[inline]
    fn from(value: &'a serde_json::Value) -> Value<'a> {
        let mut res = Value::Null;
        for (idx, value) in value.iter_recursive::<Dfs>() {
            res[&idx] = match value {
                serde_json::Value::Null => Value::Null,
                serde_json::Value::Bool(b) => Value::Bool(b),
                serde_json::Value::Number(f) => {
                    let f = OrderedFloat(f.as_f64().unwrap_or(f64::NAN));
                    Value::Number(f)
                }
                serde_json::Value::String(s) => Value::String(s.as_str()),
                serde_json::Value::Array(a) => Value::Array(vec![Value::Null; a.len()]),
                serde_json::Value::Object(_) => Value::Object(Map::default()),
            };
        }
        res
    }
}

impl<'a> PartialOrd for Map<'a> {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(std::cmp::Ord::cmp(&self, &other))
    }
}

impl<'a> std::cmp::Ord for Map<'a> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let self_iter: Vec<(&String, &Value<'a>)> = self.iter().collect();
        let other_iter: Vec<(&String, &Value<'a>)> = other.iter().collect();
        std::cmp::Ord::cmp(&self_iter, &other_iter)
    }
}

impl<'a> Eq for Map<'a> {}

impl<'a> PartialEq for Map<'a> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let self_iter: Vec<(&String, &Value<'a>)> = self.iter().collect();
        let other_iter: Vec<(&String, &Value<'a>)> = other.iter().collect();
        std::cmp::PartialEq::eq(&self_iter, &other_iter)
    }
}

pub trait Ord {
    fn cmp(&self, other: &Self) -> Ordering;
}

impl self::Ord for serde_json::Value {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        std::cmp::Ord::cmp(&Value::from(self), &Value::from(other))
    }
}

impl<'a> self::Ord for &'a serde_json::Value {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self::Ord::cmp(*self, *other)
    }
}

#[cfg(test)]
pub mod test {
    use super::{Map as SMap, Value as SV};
    use pretty_assertions::assert_eq;
    use serde_json::json;

    #[test]
    fn object_from_value() {
        let value = json!({
            "a": "a",
            "c": "c",
        });
        let expected = SV::Object(SMap::from_iter([
            (String::from("a"), SV::String("a")),
            (String::from("c"), SV::String("c")),
        ]));
        assert_eq!(&SV::from(&value), &expected);
    }

    #[test]
    fn array_from_value() {
        let value = json!([3, 2, 1]);
        let expected = SV::Array(vec![
            SV::Number(3f64.into()),
            SV::Number(2f64.into()),
            SV::Number(1f64.into()),
        ]);
        assert_eq!(&SV::from(&value), &expected);
    }

    #[test]
    fn null_from_value() {
        assert_eq!(&SV::from(&json!(null)), &SV::Null);
    }

    #[test]
    fn string_from_value() {
        assert_eq!(&SV::from(&json!("test")), &SV::String("test"));
    }

    #[test]
    fn bool_from_value() {
        assert_eq!(&SV::from(&json!(false)), &SV::Bool(&false));
    }

    #[test]
    fn number_from_value() {
        assert_eq!(&SV::from(&json!(12)), &SV::Number(12f64.into()));
        assert_eq!(&SV::from(&json!(-10)), &SV::Number((-10f64).into()));
        assert_eq!(
            &SV::from(&json!(-10.8273)),
            &SV::Number((-10.8273f64).into())
        );
        assert_eq!(&SV::from(&json!(f64::NAN)), &SV::Null);
    }

    #[test]
    fn complex_object_from_value() {
        let value = json!({
            "a": "a",
            "b": true,
            "c": null,
            "d": [3, 2, 1],
            "e": { "1": false, "2": [1, 2] },
        });
        let expected = SV::Object(SMap::from_iter([
            (String::from("a"), SV::String("a")),
            (String::from("b"), SV::Bool(&true)),
            (String::from("c"), SV::Null),
            (
                String::from("d"),
                SV::Array(vec![
                    SV::Number(3f64.into()),
                    SV::Number(2f64.into()),
                    SV::Number(1f64.into()),
                ]),
            ),
            (
                String::from("e"),
                SV::Object(SMap::from_iter([
                    (String::from("1"), SV::Bool(&false)),
                    (
                        String::from("2"),
                        SV::Array(vec![SV::Number(1f64.into()), SV::Number(2f64.into())]),
                    ),
                ])),
            ),
        ]));
        assert_eq!(&SV::from(&value), &expected);
    }

    #[test]
    fn value_ord() {
        use std::cmp::{Ord, Ordering};
        assert_eq!(Ord::cmp(&SV::String("a"), &SV::String("b")), Ordering::Less);
        assert_eq!(
            Ord::cmp(&SV::String("b"), &SV::String("a")),
            Ordering::Greater
        );
        assert_eq!(
            Ord::cmp(&SV::Number(1f64.into()), &SV::Number(2f64.into())),
            Ordering::Less
        );
        assert_eq!(
            Ord::cmp(&SV::Bool(&false), &SV::Bool(&true)),
            Ordering::Less
        );
        assert_eq!(
            Ord::cmp(
                &SV::Array(vec![SV::String("a"), SV::String("b")]),
                &SV::Array(vec![SV::String("a")])
            ),
            Ordering::Greater
        );
        assert_eq!(
            Ord::cmp(
                &SV::Array(vec![SV::Bool(&false), SV::String("a")]),
                &SV::Array(vec![SV::String("a")])
            ),
            Ordering::Less
        );
    }
}
