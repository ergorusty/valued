use crate::Value;

pub trait SerializeAs: Clone {
    fn serialize_as(&self) -> Value {
        self.clone().serialize_from()
    }

    fn serialize_from(self) -> Value;
}

// /// This trait is used in trait objects. It has more restrictions than
// /// [SerializeAs].
// pub trait ValueTrait: Debug + DynEq + DynHash {
//     fn as_value(&self) -> Value;
// }

// pub trait Resource: ValueTrait + Hash + Eq {}

// impl<T> Resource for T where T: ValueTrait + Hash + Eq {}
