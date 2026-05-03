use std::fmt::Debug;

use crate::prelude::{AccessStorage, Accessor, BlacklistStorage, ControlStorage, CredentialStorage, RegistryStorage, ReservationStorage, WhitelistStorage};

pub trait StorageTrait {
    type Value;
    type ValueId: Debug;
    type S: RegistryStorage<Value = Self::Value, ValueId = Self::ValueId>;
    
    type Access: Debug + Accessor<StoredValue = Self::Value>;
    type AS: AccessStorage<Access = Self::Access, ValueId = Self::ValueId> + Default;
    
    type ReserverId: Debug + PartialEq;
    type RS: ReservationStorage<ReserverId = Self::ReserverId, AccessStorage = Self::AS>;

    type OS: CredentialStorage<Id = Self::ReserverId>;

    type WS: WhitelistStorage<Id = Self::ValueId, Access = Self::Access>;
    type BS: BlacklistStorage<Id = Self::ValueId, Access = Self::Access>;

    type CS: ControlStorage<Id = Self::ReserverId, ResourceId = Self::ValueId>;
}