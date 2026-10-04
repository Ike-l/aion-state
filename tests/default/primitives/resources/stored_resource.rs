use crate::default::prelude::Resource;

pub type StoredResource = Resource;

// impl  for StoredResource {
//     type Value = Resource;

//     fn new(value: Self::Value) -> Self {
//         value
//     }

//     fn as_unique(&mut self) -> &mut Self::Value {
//         self
//     }

//     fn into_inner(self) -> Self::Value {
//         self
//     }
// }