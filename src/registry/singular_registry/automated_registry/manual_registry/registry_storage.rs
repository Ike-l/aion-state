pub trait RegistryStorage {
    type ValueId;

    // Accessor::StoredResource
    type Value;

    fn get(
        &self, 
        value_id: &Self::ValueId
    ) -> Option<&Self::Value>;

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
}

