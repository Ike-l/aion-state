
// if do raw ptr stuff will need StoredValue to be boxed or something
// because otherwise an insert could reallocate and create danling ptrs
pub trait Accessor {
    type StoredValue;
    type Value;

    type AccessResult<'a, T> where T: 'a;

    fn access<'a>(
        &self, 
        stored_value: &'a Self::StoredValue
    ) -> Self::AccessResult<'a, Self::Value>;

    fn remove<'a>(
        &self,
        stored_value: Self::StoredValue
    ) -> Self::AccessResult<'a, Self::StoredValue>;

    fn insert<'a>(
        &self,
        value: Self::Value
    ) -> Self::StoredValue;

    fn can_insert(&self) -> bool;
    fn can_remove(&self) -> bool;
}