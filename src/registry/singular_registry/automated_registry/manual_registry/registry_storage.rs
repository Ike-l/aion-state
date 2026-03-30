use crate::prelude::Accessor;

pub trait RegistryStorage {
    type ValueId;

    // Accessor::StoredResource
    type Value;

    fn get_mut(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<&mut Self::Value>;

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::Value
    ) -> Option<Self::Value>; 

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::Value>;

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool;

    fn release<Access: Accessor<StoredValue = Self::Value>>(
        &mut self,
        _value_id: &Self::ValueId,
        _access: &Access
    ) -> bool { true }
}

