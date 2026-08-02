use std::{fmt::Debug, hash::Hash};

use crate::prelude::{AccessStorage, Accessor, AccessorResult, BlacklistStorage, ControlStorage, CredentialStorage, Notifier, RegistryAcquireAccess, RegistryNotifiedAcquireAccess, RegistryStorage, ReservationStorage, StoredValueTrait, SynchronisedRegistryAcquireAccessError, UnsynchronisedRegistry, WhitelistStorage};

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
        S::ValueId: Hash + Eq,
        <S as RegistryStorage>::Value: StoredValueTrait,
        AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>
{
    type AccessInput = RegistryNotifiedAcquireAccess<OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>;
    type Error = SynchronisedRegistryAcquireAccessError;
    type Output = AccessResult;

    fn register_waiter(&self, input: Self::AccessInput) -> crate::prelude::sync::Arc<crate::prelude::sync::Mutex<crate::prelude::Waiter>> {
        self.notify_queue.lock().register(input.resource_id)
    }

    fn acquire_access(&self, input: Self::AccessInput) -> Result<Self::Output, Self::Error> {
        unsafe { self.acquire_access::<AccessResult>(RegistryAcquireAccess {
            user_details: input.user_details.as_ref().map(|(a, b)| { (a, b) }),
            resource_id: input.resource_id,
            access: input.access,
            password: input.password.as_ref(),
        }) }     
    }
}