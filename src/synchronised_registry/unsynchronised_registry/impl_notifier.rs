use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, AccessorResult, BlacklistStorage, ControlStorage, CredentialStorage, Notifier, RegistryNotifiedAcquireAccess, RegistryStorage, ReservationStorage, StoredValueTrait, UnsynchronisedRegistry, SynchronisedRegistryAcquireAccessError, WhitelistStorage};

impl<
    'a,
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>,
    AccessResult,
> Notifier for UnsynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor, 
        <S as RegistryStorage>::Value: StoredValueTrait,
        AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>
{
    type AccessInput = RegistryNotifiedAcquireAccess<OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>;
    type Error = SynchronisedRegistryAcquireAccessError;
    type Output = AccessResult;

    fn register_waiter(&self, input: Self::AccessInput) -> crate::prelude::sync::Arc<crate::prelude::sync::Mutex<crate::prelude::Waiter>> {
        self.notify_queue.register(input.)
    }

    fn acquire_access(&self, input: Self::AccessInput) -> Result<Self::Output, Self::Error> {
        todo!()        
    }
}