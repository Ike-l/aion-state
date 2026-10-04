use crate::prelude::ReferenceValue;

pub trait RegistryStorage {
    type ValueId;

    type OwnedValue;
    type ReferencedValue<'a>: ReferenceValue where Self: 'a;

    fn get_mut_wrapped(
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

