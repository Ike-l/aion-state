use aion_state::prelude::StoreValue;

use crate::default::prelude::Resource;

pub type StoredResource = Resource;

impl StoreValue for StoredResource {
    type Value = Resource;

    fn store(value: Self::Value) -> Self {
        value
    }

    fn take(self) -> Self::Value {
        self
    }
}