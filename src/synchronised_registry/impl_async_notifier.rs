use std::{fmt::Debug, hash::Hash};

use crate::prelude::{AccessStorage, Accessor, AccessorResult, AsyncNotifier, BlacklistStorage, ControlStorage, CredentialStorage, RegistryAcquireAccess, RegistryStorage, ReservationStorage, WrappedValue, SynchronisedRegistry, WhitelistStorage};

impl<
    'a,
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>,
> AsyncNotifier<'a, S::ReferencedValue<'a>> for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor, 
        S::ValueId: Hash + Eq,
{
    fn async_acquire_access<AccessResult: AccessorResult<'a, S::ReferencedValue<'a>>>(&'a self, input: Self::AccessInput) -> impl Future<Output = Result<AccessResult, Self::Error>> + 'a {
        async move {
            self.acquire_access_async(RegistryAcquireAccess {
                user_details: input.user_details.as_ref().map(|(a, b)| { (a, b) }),
                resource_id: input.resource_id,
                access: input.access,
                password: input.password.as_ref(),
            }).await
        }
    }
}