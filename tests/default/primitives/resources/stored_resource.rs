use crate::default::prelude::Resource;
use aion_state::prelude::StoredValueTrait;

pub type StoredResource = Box<Resource>;

impl StoredValueTrait for StoredResource {
    type Value = Resource;

    fn new(value: Self::Value) -> Self {
        Box::new(value)
    }

    fn as_shared(&self) -> &Self::Value {
        self
    }

    fn as_unique(&mut self) -> &mut Self::Value {
        self
    }

    fn into_inner(self) -> Self::Value {
        *self
    }
}