use std::fmt::Debug;

#[cfg(feature = "notifier")]
use std::hash::Hash;

use crate::prelude::{AccessStorage, Accessor, AccessorResult, AsyncReleaser, BlacklistStorage, ControlStorage, CredentialStorage, RegistryAcquireAccess, RegistryReleasingReleaseAccess, RegistryStorage, ReleasingResult, ReservationStorage, StoredValueTrait, SynchronisedRegistry, WhitelistStorage, sync::Arc};

#[cfg(not(feature = "notifier"))]
impl<
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>
> AsyncReleaser<<S::Value as StoredValueTrait>::Value> for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor + Clone,
        AS::ValueId: Clone,
        S::Value: StoredValueTrait
{
    fn async_acquire_access<'a, AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>>(
        self: &'a Arc<Self>, 
        input: Self::AccessInput
    ) -> 
        impl Future<Output = Result<ReleasingResult<<S::Value as StoredValueTrait>::Value, AccessResult, Self>, Self::AccessError>> + 'a
    {
        async move {
            let result = self
                .acquire_access_async(RegistryAcquireAccess {
                    user_details: input.user_details.as_ref().map(|(a, b)| (a, b)),
                    resource_id: input.resource_id.clone(),
                    access: input.access.clone(),
                    password: input.password.as_ref(),
                })
                .await?;

            Ok(ReleasingResult::new(
                result,
                Arc::clone(self),
                RegistryReleasingReleaseAccess {
                    resource_id: input.resource_id,
                    access: input.access,
                },
            ))
        }
    }
}

#[cfg(feature = "notifier")]
impl<
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>
> AsyncReleaser<<S::Value as StoredValueTrait>::Value> for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor + Clone,
        S::ValueId: Clone + Eq + Hash,
        S::Value: StoredValueTrait
{
    fn async_acquire_access<'a, AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>>(
        self: &'a Arc<Self>, 
        input: Self::AccessInput
    ) -> 
        impl Future<Output = Result<ReleasingResult<<S::Value as StoredValueTrait>::Value, AccessResult, Self>, Self::AccessError>> + 'a
    {
        async move {
            let result = self
                .acquire_access_async(RegistryAcquireAccess {
                    user_details: input.user_details.as_ref().map(|(a, b)| (a, b)),
                    resource_id: input.resource_id.clone(),
                    access: input.access.clone(),
                    password: input.password.as_ref(),
                })
                .await?;

            Ok(ReleasingResult::new(
                result,
                Arc::clone(self),
                RegistryReleasingReleaseAccess {
                    resource_id: input.resource_id,
                    access: input.access,
                },
            ))
        }
    }
}
