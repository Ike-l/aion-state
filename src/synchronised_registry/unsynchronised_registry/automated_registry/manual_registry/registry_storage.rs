pub trait RegistryStorage {
    type ValueId;

    type OwnedValue;
    type ReferencedValue<'a> where Self: 'a;

    /// Note:
    /// 
    /// After getting a unique reference you can then cast it to a shared reference using `Access` and `AccessorResult`
    /// 
    /// (But not the other way around)
    fn get_mut(
        &mut self,
        value_id: &Self::ValueId
    ) -> Option<Self::ReferencedValue<'_>>;

    fn insert(
        &mut self, 
        value_id: Self::ValueId, 
        value: Self::OwnedValue
    ) -> Option<Self::OwnedValue>; 

    fn remove(
        &mut self, 
        value_id: &Self::ValueId
    ) -> Option<Self::OwnedValue>;

    fn contains_key(
        &self, 
        value_id: &Self::ValueId
    ) -> bool;

    fn len(&self) -> usize;

    fn keys(&self) -> impl Iterator<Item = &Self::ValueId>;

    /// # Safety
    /// 
    /// return must guarantee semantics
    unsafe fn next_insert_may_reallocates(&self) -> bool;

    /// # Safety
    /// 
    /// return must guarantee semantics
    unsafe fn next_removal_may_reallocates(&self) -> bool;
}

