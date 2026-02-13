
pub trait Accessor {
    // input -> stored -> output
    // Value -> StoredValue -> AccessResult

    type StoredValue;
    type Value;
    type AccessResult<'a>;

    /// If a resource is currently being acessed by self; Can self also be used to access?
    fn accepts_incoming(&self, incoming_access: &Self) -> bool;

    /// Can self be used to insert a resource
    /// 
    /// In combination with `can_remove` allows `replace`
    fn can_insert(&self) -> bool;

    /// Can self be used to remove a resource
    /// 
    /// In combination with `can_insert` allows `replace`
    fn can_remove(&self) -> bool;

    /// When acquiring a resource the stored value is passed through `acquire` and the result is returned by the function
    fn acquire<'a>(
        &self, 
        stored_value: &'a Self::StoredValue
    ) -> Self::AccessResult<'a>;

    /// If `can_access(self, other)` then `merge(self, other)`
    /// 
    /// Used to collapse an incoming access
    fn merge(
        &mut self,
        incoming_access: Self
    );

    /// When an access is `deaccessed` it needs to `split` from the current access
    /// 
    /// Essentially the inverse function of `merge`
    fn release(
        &mut self,
        other: &Self
    );

    /// When inserting should the value be transformed to a stored value
    fn insert<'a>(
        &self,
        value: Self::Value
    ) -> Self::StoredValue;

    /// When removing should the stored value be transformed into a value
    /// 
    /// Essentially the inverse function of `insert`
    fn remove<'a>(
        &self,
        stored_value: Self::StoredValue
    ) -> Self::StoredValue;    
}