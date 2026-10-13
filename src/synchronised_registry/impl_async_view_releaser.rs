use std::{fmt::Debug, future::Future, hash::Hash};

use crate::prelude::{sync::Arc, AccessStorage, Accessor, AccessorResult, AsyncViewReleaser, BlacklistStorage, ControlStorage, CredentialStorage, RegistryAcquireView, RegistryReleasingRelease, RegistryStorage, ReleasingResult, ReservationStorage, SynchronisedRegistry, WhitelistStorage};

impl<
    'a,
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>,
> AsyncViewReleaser<'a, S::StorageView<'a>> for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS>
where
    RS::ReserverId: Debug + PartialEq,
    AS::Access: Accessor + Clone,
    S::ValueId: Clone + Eq + Hash,
{
    fn async_acquire_released_view<AccessResult: AccessorResult<S::StorageView<'a>>>(
        self: &'a Arc<Self>,
        input: Self::AccessInput,
    ) -> impl Future<Output = Result<ReleasingResult<'a, S::StorageView<'a>, AccessResult, Self>, Self::Error>> + 'a {
        async move {
            let release_input = RegistryReleasingRelease {
                resource_ids: input.resource_access.iter().map(|(id, _)| id.clone()).collect(),
                access: input.access.clone(),
            };
            let result = self.acquire_view_async(RegistryAcquireView {
                user_details: input.user_details.as_ref().map(|(id, password)| (id, password)),
                resource_access: input.resource_access.iter()
                    .map(|(id, password)| (id.clone(), password.as_ref()))
                    .collect(),
                access: input.access,
            }).await?;

            Ok(ReleasingResult::<S::StorageView<'a>, AccessResult, Self>::new(
                result,
                Arc::clone(self),
                release_input,
            ))
        }
    }
}
