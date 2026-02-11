
pub trait Accessor {
    // input -> stored -> output
    // Value -> StoredValue -> AccessResult

    type StoredValue;
    type Value;
    type AccessResult<'a>;

    fn can_access(&self, other: &Self) -> bool;
    fn can_insert(&self) -> bool;
    fn can_remove(&self) -> bool;

    fn acquire<'a>(
        &self, 
        stored_value: &'a Self::StoredValue
    ) -> Self::AccessResult<'a>;

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
    ) -> Self::StoredValue;    
}