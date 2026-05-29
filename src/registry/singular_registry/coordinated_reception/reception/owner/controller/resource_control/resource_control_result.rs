use crate::prelude::ResourceControlCheckOwner;

pub enum ResourceControlCheckOwnerResult {
    Verification(bool)
}

impl ResourceControlCheckOwnerResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Verification(true))
    }
}

pub enum ResourceControlReleaseResult {
    Released(bool)
}

impl ResourceControlReleaseResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Released(true))
    }
}

pub enum ResourceControlOwnResult {
    Own(bool),
    OwnershipConflict
}

pub enum ResourceControlReleaseIdResult<T: Iterator> {
    Released(T)
}

pub enum ResourceControlCheckOwnersResult<'a, Id, ResourceId> {
    Invalid(Option<ResourceControlCheckOwner<'a, Id, ResourceId>>)
}

impl<Id, ResourceId> ResourceControlCheckOwnersResult<'_, Id, ResourceId> {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Invalid(None))
    }
}

pub enum ResourceControlIsOwnedResult {
    IsOwned(bool)
}