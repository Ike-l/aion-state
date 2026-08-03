use std::{fmt::Debug, hash::Hash};

use crate::prelude::{sync::{Arc, Mutex}, AccessStorage, Accessor, AccessorResult, BlacklistStorage, ControlStorage, CredentialStorage, Notifier, RegistryAcquireAccess, RegistryNotifiedAcquireAccess, RegistryStorage, ReservationStorage, StoredValueTrait, SynchronisedRegistry, SynchronisedRegistryAcquireAccessError, Waiter, WhitelistStorage};

impl<
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>,
> Notifier<<<S as RegistryStorage>::Value as StoredValueTrait>::Value> for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor, 
        S::ValueId: Hash + Eq,
        <S as RegistryStorage>::Value: StoredValueTrait,
{
    type AccessInput = RegistryNotifiedAcquireAccess<OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>;
    type Error = SynchronisedRegistryAcquireAccessError;

    fn register_waiter(&self, input: Self::AccessInput) -> Arc<Mutex<crate::prelude::Waiter>> {
        self.notify_queue.lock().register(input.resource_id)
    }

    fn unregister_waiter(&self, input: &Self::AccessInput, waiter: &Arc<Mutex<Waiter>>) {
        self.notify_queue.lock().unregister(&input.resource_id, waiter);
    }

    fn acquire_access<'a, AccessResult: AccessorResult<'a, <<S as RegistryStorage>::Value as StoredValueTrait>::Value>>(&'a self, input: Self::AccessInput) -> Result<AccessResult, Self::Error> {
        self.acquire_access(RegistryAcquireAccess {
            user_details: input.user_details.as_ref().map(|(a, b)| { (a, b) }),
            resource_id: input.resource_id,
            access: input.access,
            password: input.password.as_ref(),
        })   
    }
}