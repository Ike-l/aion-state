use std::fmt::Debug;

use crate::prelude::{sync::Arc, AccessStorage, Accessor, AccessorResult, BlacklistStorage, ControlStorage, CredentialStorage, RegistryAcquireView, RegistryOwnedAcquireView, RegistryReleasingRelease, RegistryStorage, ReleasingResult, ReservationStorage, SynchronisedRegistry, SynchronisedRegistryAcquireViewError, ViewReleaser, WhitelistStorage};

use std::hash::Hash;

impl<
    'a,
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>,
> ViewReleaser<'a, S::StorageView<'a>> for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS>
where
    RS::ReserverId: Debug + PartialEq,
    AS::Access: Accessor + Clone,
    S::ValueId: Clone + Eq + Hash,
{
    type Error = SynchronisedRegistryAcquireViewError;
    type AccessInput = RegistryOwnedAcquireView<OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>;

    fn acquire_released_view<AccessResult: AccessorResult<S::StorageView<'a>>>(
        self: &'a Arc<Self>,
        input: Self::AccessInput,
    ) -> Result<ReleasingResult<'a, S::StorageView<'a>, AccessResult, Self>, Self::Error> {
        let release_input = RegistryReleasingRelease {
            resource_ids: input.resource_access.iter().map(|(id, _)| id.clone()).collect(),
            access: input.access.clone(),
        };
        let result = self.acquire_view(RegistryAcquireView {
            user_details: input.user_details.as_ref().map(|(id, password)| (id, password)),
            resource_access: input.resource_access.iter()
                .map(|(id, password)| (id.clone(), password.as_ref()))
                .collect(),
            access: input.access,
        })?;

        Ok(ReleasingResult::<S::StorageView<'a>, AccessResult, Self>::new(
            result,
            Arc::clone(self),
            release_input,
        ))
    }
}
