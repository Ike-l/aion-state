use std::fmt::Debug;
use std::hash::Hash;

use crate::prelude::{sync::Arc, AccessStorage, Accessor, AccessorResult, BlacklistStorage, ControlStorage, CredentialStorage, RegistryAcquireAccess, RegistryReleaseAccess, RegistryOwnedAcquireAccess, RegistryReleasingReleaseAccess, RegistryStorage, Releaser, ReleasingResult, ReservationStorage, StoredValueTrait, UnsynchronisedRegistry, UnsynchronisedRegistryAcquireAccessError, WhitelistStorage};

impl<
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>
> Releaser<<S::Value as StoredValueTrait>::Value> for UnsynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor + Clone,
        S::ValueId: Clone + Eq + Hash,
        S::Value: StoredValueTrait
{
    type Error = UnsynchronisedRegistryAcquireAccessError;
    type AccessInput = RegistryOwnedAcquireAccess<OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>;

    type ReleaseInput = RegistryReleasingReleaseAccess<S::ValueId, AS::Access>;

    fn acquire_released_access<'a, AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>>(self: &'a Arc<Self>, input: Self::AccessInput) -> Result<ReleasingResult<<S::Value as StoredValueTrait>::Value, AccessResult, Self>, Self::Error> {
        let result = unsafe { self.as_ref().acquire_access(RegistryAcquireAccess {
            user_details: input.user_details.as_ref().map(|(a, b)| { (a, b) }),
            resource_id: input.resource_id.clone(),
            access: input.access.clone(),
            password: input.password.as_ref()
        }) }?;

        Ok(ReleasingResult::new(result, Arc::clone(self), RegistryReleasingReleaseAccess {
            resource_id: input.resource_id,
            access: input.access
        }))
    }

    fn release_access(&self, input: &Self::ReleaseInput) {
        unsafe { self.release_access(&RegistryReleaseAccess {
            resource_id: &input.resource_id,
            access: &input.access
        }) };
    }
}