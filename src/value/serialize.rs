use linked_hash_map::LinkedHashMap as OrderedMap;
use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize,
};
use std::hash::Hash;

use crate::{value::primitive::Primitive, Value};

impl Serialize for Primitive {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Primitive::String(string) => string.serialize(serializer),
            Primitive::U64(n) => n.serialize(serializer),
            Primitive::I64(n) => n.serialize(serializer),
            Primitive::Decimal(n) => n.serialize(serializer),
        }
    }
}

impl Serialize for Value {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Value::Primitive(value) => value.serialize(serializer),
            Value::Seq(list) => {
                let mut seq = serializer.serialize_seq(Some(list.len()))?;

                for item in list {
                    seq.serialize_element(item)?;
                }
                seq.end()
            }
            Value::Dict(entries) => serialize_entries(entries, serializer),
            Value::Map(entries) => serialize_entries(entries, serializer),
        }
    }
}

fn serialize_entries<S>(
    ordered_map: &OrderedMap<impl Serialize + Hash + Eq, impl Serialize>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut map = serializer.serialize_map(Some(ordered_map.len()))?;

    for (k, v) in ordered_map.iter() {
        map.serialize_entry(k, v)?;
    }

    map.end()
}
