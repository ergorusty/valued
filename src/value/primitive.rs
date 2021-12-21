use bigdecimal::BigDecimal;
use std::hash::Hash;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
#[non_exhaustive]
pub enum Primitive {
    String(String),
    U64(u64),
    I64(i64),
    Decimal(BigDecimal),
}

macro_rules! primitive {
    (Primitive::$to:ident from $($from:ty),* $(,)?) => {
        $(
            impl From<$from> for Primitive {
                fn from(from: $from) -> Self {
                    Primitive::$to(from.into())
                }
            }

            impl From<$from> for $crate::Value {
                fn from(from: $from) -> Self {
                    $crate::Value::Primitive(from.into())
                }
            }

            impl $crate::SerializeAs for $from {
                fn serialize_from(self) -> crate::Value {
                    $crate::Value::Primitive(self.into())
                }
            }
        )*
    };
}

primitive!(Primitive::String from &str, &String, String);
primitive!(Primitive::U64 from u32, u64);
primitive!(Primitive::I64 from i32, i64);
primitive!(Primitive::Decimal from BigDecimal);
