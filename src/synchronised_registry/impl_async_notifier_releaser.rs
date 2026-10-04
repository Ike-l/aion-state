use std::{fmt::Debug, hash::Hash};

use crate::prelude::{AccessStorage, Accessor, AsyncNotifiedReleaser, BlacklistStorage, ControlStorage, CredentialStorage, RegistryOwnedAcquireAccess, RegistryStorage, ReservationStorage, WrappedValue, SynchronisedRegistry, SynchronisedRegistryAcquireAccessError, WhitelistStorage};

impl<
    'a,
    S: RegistryStorage,
    RS: ReservationStorage<AccessStorage = AS>,
    AS: AccessStorage<ValueId = S::ValueId> + Default,
    OS: CredentialStorage<Id = RS::ReserverId>,
    WS: WhitelistStorage<Id = AS::ValueId, Access = AS::Access>,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = OS::Id, ResourceId = BS::Id>
> AsyncNotifiedReleaser<'a, S::ReferencedValue<'a>, RegistryOwnedAcquireAccess<OS::Id, OS::Password, S::ValueId, AS::Access, BS::Password>, SynchronisedRegistryAcquireAccessError> for SynchronisedRegistry<S, RS, AS, OS, WS, BS, CS> 
    where 
        RS::ReserverId: Debug + PartialEq,
        AS::Access: Accessor + Clone,
        S::ValueId: Clone + Eq + Hash,
        S::ReferencedValue<'a>: WrappedValue,
{}
