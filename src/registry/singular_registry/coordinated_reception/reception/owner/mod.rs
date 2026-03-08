use crate::prelude::{AuthenticateInput, Authenticator, BlacklistStorage, ControlStorage, Controller, ControllerOwn, CredentialStorage, OwnerOwn, OwnerOwnResult, WhitelistStorage};

pub mod authenticator;
pub mod controller;

pub mod owner_result;
pub mod owner_input;

/// Applies `Authentication` semantics when ownership of the door is required, then `Door` semantics 
pub struct Owner<AS, WS, BS, CS> {
    authenticator: Authenticator<AS>,
    controller: Controller<WS, BS, CS>,
}

impl<
    AS: CredentialStorage,
    WS: WhitelistStorage,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>,
    CS: ControlStorage<Id = AS::Id, ResourceId = WS::Id>,
> Owner<AS, WS, BS, CS> {
    pub fn register() {}

    pub fn update_password() {}

    pub fn own(
        &mut self,
        OwnerOwn {
            id, resource_id, password
        }: OwnerOwn<'_, AS::Id, WS::Id, AS::Password>
    ) -> OwnerOwnResult {
        if self.authenticator.authenticate(AuthenticateInput { id: &id, password }).ok() {
            OwnerOwnResult::Controller(self.controller.own(ControllerOwn { id, resource_id }))
        } else {
            OwnerOwnResult::Denied
        }
    }

    pub fn release() {}

    pub fn allow_whitelist() {}

    pub fn allow_blacklist() {}

    pub fn access() {}
}