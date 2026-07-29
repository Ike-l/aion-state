use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, AccessorResult, BlacklistStorage, ControlStorage, CredentialStorage, RegistryAcquireAccess, RegistryReleaseAccess, RegistryReleasingAcquireAccess, RegistryReleasingReleaseAccess, RegistryStorage, Releaser, ReleasingResult, ReservationStorage, StoredValueTrait, SynchronisedRegistry, SynchronisedRegistryAcquireAccessError, WhitelistStorage};

impl<
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>
> Releaser<<S::Value as StoredValueTrait>::Value> for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor + Clone,
        AS::ValueId: Clone,
        S::Value: StoredValueTrait
{
    type AccessError = SynchronisedRegistryAcquireAccessError;
    type AccessInput = RegistryReleasingAcquireAccess<OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>;

    type ReleaseInput = RegistryReleasingReleaseAccess<S::ValueId, AS::Access>;

    // because the import is from prelude
    #[allow(clippy::disallowed_types)]
    fn acquire_access<'a, AccessResult: AccessorResult<'a, <S::Value as StoredValueTrait>::Value>>(self: &'a crate::prelude::sync::Arc<Self>, input: Self::AccessInput) -> Result<ReleasingResult<<S::Value as StoredValueTrait>::Value, AccessResult, Self>, Self::AccessError> {
        let result = self.as_ref().acquire_access(RegistryAcquireAccess {
            user_details: input.user_details.as_ref().map(|(a, b)| { (a, b) }),
            resource_id: input.resource_id.clone(),
            access: input.access.clone(),
            password: input.password.as_ref()
        })?;

        // because the import is from prelude
        #[allow(clippy::disallowed_types)]
        Ok(ReleasingResult::new(result, crate::prelude::sync::Arc::clone(self), RegistryReleasingReleaseAccess {
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
