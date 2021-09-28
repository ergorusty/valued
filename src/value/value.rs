use giftwrap::Wrap;
use linked_hash_map::LinkedHashMap as OrderedMap;
use std::iter::empty;

use crate::{value::primitive::Primitive, SerializeAs};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Wrap)]
pub enum Value {
    Primitive(Primitive),
    Seq(Vec<Value>),
    Dict(OrderedMap<String, Value>),
    Map(OrderedMap<Value, Value>),
    // TODO: streamy, iterable items -- not needed atm in for omakase
}

impl SerializeAs for Value {
    fn serialize_as(&self) -> Value {
        self.clone()
    }

    fn serialize_from(self) -> Value {
        self
    }
}

impl Value {
    pub fn list(iterator: impl IntoIterator<Item = impl SerializeAs>) -> Value {
        Value::Seq(iterator.into_iter().map(|i| i.serialize_from()).collect())
    }
}

#[allow(non_snake_case)]
pub fn Value(value: impl Into<Value>) -> Value {
    value.into()
}

#[allow(non_snake_case)]
pub fn Seq(list: impl IntoIterator<Item = impl SerializeAs>) -> Value {
    Value::Seq(list.into_iter().map(|i| i.serialize_from()).collect())
}

#[allow(non_snake_case)]
pub fn Dict(list: impl IntoIterator<Item = (impl Into<String>, impl Into<Value>)>) -> Value {
    Value::Dict(
        list.into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
    )
}

pub struct EMPTY;

impl IntoIterator for EMPTY {
    type Item = (String, Value);

    type IntoIter = std::iter::Empty<(String, Value)>;

    fn into_iter(self) -> Self::IntoIter {
        empty()
    }
}

#[allow(non_snake_case)]
pub fn Map(list: impl IntoIterator<Item = (impl Into<Value>, impl Into<Value>)>) -> Value {
    Value::Map(
        list.into_iter()
            .map(|(k, v)| (k.into(), v.into()))
            .collect(),
    )
}
