pub trait RegistryStorage {
    type ValueId;

    // Accessor::StoredResource
    type Value;

    fn get(
        &self, 
        key: &Self::ValueId
    ) -> Option<&Self::Value>;

    fn insert(
        &mut self, 
        key: Self::ValueId, 
        value: Self::Value
    ) -> Option<Self::Value>; 

    fn remove(
        &mut self, 
        key: &Self::ValueId
    ) -> Option<Self::Value>;

    fn contains_key(
        &self, 
        key: &Self::ValueId
    ) -> bool;
}

