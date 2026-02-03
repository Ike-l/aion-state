
// if do raw ptr stuff will need StoredValue to be boxed or something
// because otherwise an insert could reallocate and create danling ptrs
pub trait Accessor {
    type StoredValue;
    type Value;

    type AccessResult<'a, T> where T: 'a;

    fn can_access(&self, other: &Self) -> bool;
    fn can_insert(&self) -> bool;
    fn can_remove(&self) -> bool;

    // if the container containing self can be reallocated
    fn can_reallocate(&self) -> bool;

    fn acquire<'a>(
        &self, 
        stored_value: &'a Self::StoredValue
    ) -> Self::AccessResult<'a, Self::Value>;

    fn merge(
        &mut self,
        other: Self
    );

    fn release(
        &mut self,
        other: &Self
    );

    fn insert<'a>(
        &self,
        value: Self::Value
    ) -> Self::StoredValue;

    fn remove<'a>(
        &self,
        stored_value: Self::StoredValue
    ) -> Self::AccessResult<'a, Self::StoredValue>;    
}