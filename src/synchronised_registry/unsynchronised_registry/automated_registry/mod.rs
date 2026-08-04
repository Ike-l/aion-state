use std::cell::UnsafeCell;

use stable_deref_trait::StableDeref;

use crate::prelude::{Accessor, AccessorResult, ManualRegistry, ManualRegistryAccessError, ManualRegistryAccessInput, ManualRegistryCheckedReplacementResult, ManualRegistryReplacementInput, ManualRegistryReplacementResult, RegistryStorage, StoredValueTrait, trace_function};

pub mod manual_registry;

/// Provides individual access ability to the underlying storage
/// 
/// Requires `Automation` to ensure safety
#[derive(Default)]
pub struct AutomatedRegistry<S> {
    manual_registry: UnsafeCell<ManualRegistry<S>>
}

impl<S: RegistryStorage> AutomatedRegistry<S> {
    /// # Safety 
    /// 
    /// No Concurrent Unique References
    unsafe fn get_inner(&self) -> &ManualRegistry<S> {
        unsafe { & *self.manual_registry.get() }
    }

    /// # Safety 
    /// 
    /// No Concurrent References
    unsafe fn get_inner_mut(&self) -> &mut ManualRegistry<S> {
        unsafe { &mut *self.manual_registry.get() }
    }

    /// # Safety 
    /// 
    /// No Concurrent Unique References
    pub unsafe fn acquire_access<'a, Access: Accessor, AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>>(
        &'a self,
        input: ManualRegistryAccessInput<'_, S::ValueId, Access>
    ) -> Result<AccessResult, ManualRegistryAccessError> 
        where <S as RegistryStorage>::Value: StoredValueTrait 
    {
        trace_function!("Automated Registry Acquire Access");

        unsafe { self.get_inner_mut() }.acquire_access(input)
    }

    /// # Safety 
    /// 
    /// No Concurrent References
    /// 
    /// Access cannot be used to 'acquire' a reference to StableAddress itself
    /// 
    /// Insert won't invalidate concurrent access
    /// 
    /// ^ i.e do not replace a borrowed item
    pub unsafe fn reallocating_replace<Access: Accessor>(
        &self,
        manual_registry_replacement_input: ManualRegistryReplacementInput<'_, Access, S::ValueId, <S::Value as StoredValueTrait>::Value>
    ) -> ManualRegistryReplacementResult<<S::Value as StoredValueTrait>::Value> 
        where <S as RegistryStorage>::Value: StableDeref + StoredValueTrait
    {
        trace_function!("Automated Reallocating Replacement");

        unsafe { self.get_inner_mut().reallocating_replace(manual_registry_replacement_input) }
    }

    /// # Safety 
    /// 
    /// No Concurrent References
    /// 
    /// Insert won't invalidate concurrent access
    /// 
    /// ^ i.e do not replace a borrowed item
    pub unsafe fn checked_replace<Access: Accessor>(
        &self,
        manual_registry_replacement_input: ManualRegistryReplacementInput<'_, Access, S::ValueId, <S::Value as StoredValueTrait>::Value>
    ) -> ManualRegistryCheckedReplacementResult<<S::Value as StoredValueTrait>::Value> 
        where <S as RegistryStorage>::Value: StoredValueTrait
    {
        trace_function!("Automated Checked Replacement");

        unsafe { self.get_inner_mut().checked_replace(manual_registry_replacement_input) }
    }

    /// # Safety
    /// 
    /// No Concurrent Unique References (No writes that could modify the keys)
    pub unsafe fn contains_key(
        &self,
        key: &S::ValueId
    ) -> bool {
        trace_function!("Automated Registry Contains Key");

        unsafe { self.get_inner() }.contains_key(key)
    }

    /// # Safety
    /// 
    /// No Concurrent Unique References (No writes that could modify the len)
    pub unsafe fn len(&self) -> usize {
        trace_function!("Automated Registry Len");

        unsafe { self.get_inner() }.len()
    }
}

impl<
    S: RegistryStorage,
> AutomatedRegistry<S> 
    where S::ValueId: Clone
{
    /// # Safety
    /// 
    /// No Concurrent Unique References (No writes that could modify the keys)
    pub unsafe fn keys(&self) -> Vec<S::ValueId> {
        trace_function!("Automated Registry keys");

        unsafe { self.get_inner() }.keys()
    }
}