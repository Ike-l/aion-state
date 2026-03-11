use crate::prelude::{AuthenticateRegister, AuthenticateUpdatePassword, Authentication, Authenticator, BlacklistStorage, ControlStorage, Controller, ControllerOwn, ControllerRelease, ControllerReleaseId, CredentialStorage, OwnerOwn, OwnerOwnResult, OwnerRegister, OwnerRegisterResult, OwnerRelease, OwnerReleaseResult, OwnerUnregister, OwnerUnregisterResult, OwnerUpdatePassword, OwnerUpdatePasswordResult, WhitelistStorage};

pub mod authenticator;
pub mod controller;

pub mod owner_result;
pub mod owner_input;

/// Applies `authenticator` semantics and then `controller` semantics
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
    pub fn register(
        &mut self,
        OwnerRegister {
            id, password
        }: OwnerRegister<AS::Id, AS::Password>
    ) -> OwnerRegisterResult {
        OwnerRegisterResult::Authenticator(self.authenticator.register(AuthenticateRegister { id, password }))
    }

    pub fn unregister(
        &mut self,
        OwnerUnregister {
            id, password
        }: OwnerUnregister<'_, AS::Id, AS::Password>
    ) -> OwnerUnregisterResult {
        if self.authenticator.authenticate(Authentication { id, password }).ok() {
            let authenticator_unregister = self.authenticator.unregister(AuthenticateUnregister { id });
            if authenticator_unregister.ok() {
                OwnerUnregisterResult::Controller(self.controller.release_id(ControllerReleaseId { id }))
            } else {
                OwnerUnregisterResult::Unauthorised
            }
        } else {
            OwnerUnregisterResult::Denied
        }
    }

    // this is the layer which checks passwords- why i chose this to be the layer to `authenticate`
    pub fn update_password(
        &mut self,
        OwnerUpdatePassword {
            id, old_password, new_password
        }: OwnerUpdatePassword<AS::Id, AS::Password>
    ) -> OwnerUpdatePasswordResult {
        if self.authenticator.authenticate(Authentication { id, password: old_password }).ok() {
            return OwnerUpdatePasswordResult::Authenticator(self.authenticator.update_password(AuthenticateUpdatePassword { id, new_password }))
        }

        OwnerUpdatePasswordResult::Denied
    }

    pub fn own(
        &mut self,
        OwnerOwn {
            id, resource_id, password
        }: OwnerOwn<'_, AS::Id, WS::Id, AS::Password>
    ) -> OwnerOwnResult {
        if self.authenticator.authenticate(Authentication { id: &id, password }).ok() {
            OwnerOwnResult::Controller(self.controller.own(ControllerOwn { id, resource_id }))
        } else {
            OwnerOwnResult::Denied
        }
    }

    pub fn release(
        &mut self,
        OwnerRelease {
            id, password, resource_id
        }: OwnerRelease<AS::Id, AS::Password, CS::ResourceId>
    ) -> OwnerReleaseResult {
        if self.authenticator.authenticate(Authentication { id, password }).ok() {
            return OwnerReleaseResult::Controller(self.controller.release(ControllerRelease { id, resource_id }))
        }

        OwnerReleaseResult::Denied
    }

    pub fn allow_whitelist() {}

    pub fn allow_blacklist() {}

    pub fn access() {}
}