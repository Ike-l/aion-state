
pub trait Accessor {
    /// If a resource is currently being acessed by self; Can self also be used to access?
    fn accepts_incoming(&self, incoming_access: &Self) -> bool;

    /// Can self be used to insert a resource
    /// 
    /// In combination with `can_remove` allows `replace`
    fn can_insert_resource(&self) -> bool;

    /// Can self be used to remove a resource
    /// 
    /// In combination with `can_insert` allows `replace`
    fn can_remove_resource(&self) -> bool;

    /// When acquiring a resource the stored value is passed through `acquire` and the result is returned by the function
    fn acquire<'a, V: WrappedValue, R: AccessorResult<V>>(
        &self, 
        stored_value: V
    ) -> Option<R>;

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
        subtractor: &Self
    );
}

pub trait StoreValue {
    type Value;
    fn store(value: Self::Value) -> Self;    
    fn take(self) -> Self::Value;
}

pub trait WrappedValue {
    type Value;
    fn as_unique(&mut self) -> &mut Self::Value;
}

pub trait AccessorResult<T> {
    fn to_shared(value: T) -> Self;
    fn new_unique(value: T) -> Self;
    fn new_owned(value: T) -> Self;
}