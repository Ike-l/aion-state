use std::fmt::Debug;
use std::hash::Hash;

use crate::prelude::{sync::Arc, AccessStorage, Accessor, AccessorResult, BlacklistStorage, ControlStorage, CredentialStorage, RegistryAcquireAccess, RegistryReleaseAccess, RegistryOwnedAcquireAccess, RegistryReleasingRelease, RegistryStorage, Release, Releaser, ReleasingResult, ReservationStorage, UnsynchronisedRegistry, UnsynchronisedRegistryAcquireAccessError, WhitelistStorage};

impl<
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>
> Release for UnsynchronisedRegistry<S, RS, AS, OS, WS, BS, CS>
where
    RS::ReserverId: Debug + PartialEq,
    AS::Access: Accessor + Clone,
    S::ValueId: Clone + Eq + Hash,
{
    type ReleaseInput = RegistryReleasingRelease<S::ValueId, AS::Access>;

    fn release_access(&self, input: &Self::ReleaseInput) {
        for resource_id in &input.resource_ids {
            unsafe {
                self.release_access(&RegistryReleaseAccess {
                    resource_id,
                    access: &input.access,
                });
            }
        }
    }
}

impl<
    'a,
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>
> Releaser<'a, S::ReferencedValue<'a>> for UnsynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor + Clone,
        S::ValueId: Clone + Eq + Hash,
{
    type Error = UnsynchronisedRegistryAcquireAccessError;
    type AccessInput = RegistryOwnedAcquireAccess<OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>;

    fn acquire_released_access<AccessResult: AccessorResult<S::ReferencedValue<'a>>>(self: &'a Arc<Self>, input: Self::AccessInput) -> Result<ReleasingResult<'a, S::ReferencedValue<'a>, AccessResult, Self>, Self::Error> {
        let result = unsafe { self.as_ref().acquire_access(RegistryAcquireAccess {
            user_details: input.user_details.as_ref().map(|(a, b)| { (a, b) }),
            resource_id: input.resource_id.clone(),
            access: input.access.clone(),
            password: input.password.as_ref()
        }) }?;

        Ok(ReleasingResult::<S::ReferencedValue<'a>, AccessResult, Self>::new(result, Arc::clone(self), RegistryReleasingRelease {
            resource_ids: vec![input.resource_id],
            access: input.access
        }))
    }
}
