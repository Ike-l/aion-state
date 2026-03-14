use crate::prelude::{AuthenticateRegister, AuthenticateUnregister, AuthenticateUpdatePassword, Authentication, Authenticator, BlacklistStorage, ControlStorage, Controller, ControllerAccess, ControllerAllow, ControllerOwn, ControllerRelease, ControllerReleaseId, CredentialStorage, OwnerAccess, OwnerAccessResult, OwnerAllow, OwnerBlacklistAllowResult, OwnerOwn, OwnerOwnResult, OwnerRegister, OwnerRegisterResult, OwnerRelease, OwnerReleaseResult, OwnerUnregister, OwnerUnregisterResult, OwnerUpdatePassword, OwnerUpdatePasswordResult, OwnerWhitelistAllowResult, WhitelistStorage, trace_function};

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
    /// Register the `id` with the `password`
    ///
    /// Such that to identify as `id`- `password` is required
    pub fn register(
        &mut self,
        OwnerRegister {
            id, password
        }: OwnerRegister<AS::Id, AS::Password>
    ) -> OwnerRegisterResult {
        trace_function!("Owner Register");

        OwnerRegisterResult::Authenticator(self.authenticator.register(AuthenticateRegister { id, password }))
    }

    /// Unregister `yourself`
    /// 
    /// Then releases all `control` associated with `id`
    pub fn unregister(
        &mut self,
        OwnerUnregister {
            id, password
        }: OwnerUnregister<'_, AS::Id, AS::Password>
    ) -> OwnerUnregisterResult {
        trace_function!("Owner Unregister");

        if self.authenticator.authenticate(Authentication { id, password }).ok() {
            let authenticator_unregister = self.authenticator.unregister(AuthenticateUnregister { id });
            if authenticator_unregister.ok() {
                return OwnerUnregisterResult::Controller(self.controller.release_id(ControllerReleaseId { id }))
            } else {
                return OwnerUnregisterResult::Authenticator(authenticator_unregister)
            }
        }

        OwnerUnregisterResult::Denied
    }

    // this is the layer which checks passwords- why i chose this to be the layer to `authenticate`
    /// update `your` password to `new_password`
    pub fn update_password(
        &mut self,
        OwnerUpdatePassword {
            id, old_password, new_password
        }: OwnerUpdatePassword<AS::Id, AS::Password>
    ) -> OwnerUpdatePasswordResult {
        trace_function!("Owner Update Password");

        if self.authenticator.authenticate(Authentication { id, password: old_password }).ok() {
            return OwnerUpdatePasswordResult::Authenticator(self.authenticator.update_password(AuthenticateUpdatePassword { id, new_password }))
        }

        OwnerUpdatePasswordResult::Denied
    }

    /// Take `controller` ownership over `resource_id`
    pub fn own(
        &mut self,
        OwnerOwn {
            id, password, resource_id
        }: OwnerOwn<'_, AS::Id, WS::Id, AS::Password>
    ) -> OwnerOwnResult {
        trace_function!("Owner Own");

        if self.authenticator.authenticate(Authentication { id: &id, password }).ok() {
            return OwnerOwnResult::Controller(self.controller.own(ControllerOwn { id, resource_id }))
        }

        OwnerOwnResult::Denied
    }

    /// relinquish `resource_id` from `controller`
    pub fn release(
        &mut self,
        OwnerRelease {
            id, password, resource_id
        }: OwnerRelease<AS::Id, AS::Password, CS::ResourceId>
    ) -> OwnerReleaseResult {
        trace_function!("Owner Release");

        if self.authenticator.authenticate(Authentication { id, password }).ok() {
            return OwnerReleaseResult::Controller(self.controller.release(ControllerRelease { id, resource_id }))
        }

        OwnerReleaseResult::Denied
    }

    /// create an allowance over `whitelist` semantics
    /// 
    /// for `resource_id` with `access`
    pub fn allow_whitelist(
        &mut self,
        OwnerAllow {
            id, password, resource_id, access
        }: OwnerAllow<'_, AS::Id, AS::Password, CS::ResourceId, WS::Access>
    ) -> OwnerWhitelistAllowResult {
        trace_function!("Owner Allow Whitelist");

        if self.authenticator.authenticate(Authentication { id, password }).ok() {
            return OwnerWhitelistAllowResult::Controller(self.controller.allow_whitelist(ControllerAllow { id, resource_id, access }))
        }

        OwnerWhitelistAllowResult::Denied
    }

    /// create an allowance over `blacklist` semantics
    /// 
    /// for `resource_id` with `access`
    pub fn allow_blacklist(
        &mut self,
        OwnerAllow {
            id, password, resource_id, access
        }: OwnerAllow<'_, AS::Id, AS::Password, CS::ResourceId, WS::Access>
    ) -> OwnerBlacklistAllowResult<BS::Password> {
        trace_function!("Owner Allow Blacklist");

        if self.authenticator.authenticate(Authentication { id, password }).ok() {
            return OwnerBlacklistAllowResult::Controller(self.controller.allow_blacklist(ControllerAllow { id, resource_id, access }))
        }

        OwnerBlacklistAllowResult::Denied
    }

    /// Check if `resource_id` can be accessed with `access`
    /// 
    /// If a `password` exists then it will check the `blacklist`
    /// 
    /// Otherwise will check the `whitelist`
    /// 
    /// If the `password` check fails `on` `blacklist` it will not check `whitelist`
    pub fn check_access(
        &self,
        OwnerAccess {
            id, resource_id, access, password
        }: OwnerAccess<'_, AS::Id, WS::Id, WS::Access, BS::Password>
    ) -> OwnerAccessResult {
        trace_function!("Owner Check Access");

        OwnerAccessResult::Controller(self.controller.check_access(ControllerAccess { id, resource_id, access, password }))
    }

    pub fn block_whitelist() {}
    pub fn block_blacklist() {}
    pub fn release_all() {}
}