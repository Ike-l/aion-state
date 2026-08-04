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

    fn len(&self) -> usize;

    fn keys(&self) -> impl Iterator<Item = &Self::ValueId>;

    fn next_insert_reallocates(&self) -> bool;
    fn next_removal_reallocates(&self) -> bool;
}

