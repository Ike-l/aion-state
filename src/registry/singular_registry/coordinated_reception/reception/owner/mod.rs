use crate::prelude::{AuthenticateRegister, AuthenticateUnregister, AuthenticateUpdatePassword, Authentication, Authenticator, BlacklistStorage, ControlStorage, Controller, ControllerAllow, ControllerCheckAccess, ControllerOwn, ControllerReleaseId, ControllerReleaseResource, ControllerUnallow, CredentialStorage, OwnerAllow, OwnerBlacklistAllowResult, OwnerCheckAccess, OwnerCheckAccessResult, OwnerOwn, OwnerOwnResult, OwnerRegister, OwnerRegisterResult, OwnerReleaseResource, OwnerReleaseResourceAll, OwnerReleaseResourceAllResult, OwnerReleaseResourceResult, OwnerUnallow, OwnerUnallowBlacklistResult, OwnerUnallowWhitelistResult, OwnerUnregister, OwnerUnregisterResult, OwnerUpdatePassword, OwnerUpdatePasswordResult, OwnerWhitelistAllowResult, WhitelistStorage, trace_function};

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
        }: &OwnerUnregister<'_, AS::Id, AS::Password>
    ) -> OwnerUnregisterResult {
        trace_function!("Owner Unregister");

        let authentication_result = self.authenticator.authenticate(&Authentication { id, password });
        
        if authentication_result.ok() {
            let authenticator_unregister = self.authenticator.unregister(&AuthenticateUnregister { id });
            
            if authenticator_unregister.ok() {
                return OwnerUnregisterResult::Controller(self.controller.release_id(&ControllerReleaseId { id }))
            } else {
                return OwnerUnregisterResult::Authenticator(authenticator_unregister)
            }
        }

        OwnerUnregisterResult::Denied(authentication_result)
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

        let authentication_result = self.authenticator.authenticate(&Authentication { id, password: old_password });
            
        if authentication_result.ok() {
            return OwnerUpdatePasswordResult::Authenticator(self.authenticator.update_password(AuthenticateUpdatePassword { id, new_password }))
        }

        OwnerUpdatePasswordResult::Denied(authentication_result)
    }

    /// Take `controller` ownership over `resource_id`
    pub fn own(
        &mut self,
        OwnerOwn {
            id, password, resource_id
        }: OwnerOwn<'_, AS::Id, AS::Password, WS::Id>
    ) -> OwnerOwnResult {
        trace_function!("Owner Own");

        let authentication_result = self.authenticator.authenticate(&Authentication { id: &id, password });
        
        if authentication_result.ok() {
            return OwnerOwnResult::Controller(self.controller.own(ControllerOwn { id, resource_id }))
        }

        OwnerOwnResult::Denied(authentication_result)
    }

    /// relinquish `resource_id` from `controller`
    pub fn release_resource(
        &mut self,
        OwnerReleaseResource {
            id, password, resource_id
        }: &OwnerReleaseResource<AS::Id, AS::Password, CS::ResourceId>
    ) -> OwnerReleaseResourceResult {
        trace_function!("Owner Release Resource");

        let authentication_result = self.authenticator.authenticate(&Authentication { id, password });

        if authentication_result.ok() {
            return OwnerReleaseResourceResult::Controller(self.controller.release_resource(&ControllerReleaseResource { id, resource_id }))
        }

        OwnerReleaseResourceResult::Denied(authentication_result)
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

        let authentication_result = self.authenticator.authenticate(&Authentication { id, password });
        
        if authentication_result.ok() {
            return OwnerWhitelistAllowResult::Controller(self.controller.allow_whitelist(ControllerAllow { id, resource_id, access }))
        }

        OwnerWhitelistAllowResult::Denied(authentication_result)
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

        let authentication_result = self.authenticator.authenticate(&Authentication { id, password });
        
        if authentication_result.ok() {
            return OwnerBlacklistAllowResult::Controller(self.controller.allow_blacklist(ControllerAllow { id, resource_id, access }))
        }

        OwnerBlacklistAllowResult::Denied(authentication_result)
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
        OwnerCheckAccess {
            id, resource_id, access, password
        }: &OwnerCheckAccess<'_, AS::Id, WS::Id, WS::Access, BS::Password>
    ) -> OwnerCheckAccessResult {
        trace_function!("Owner Check Access");

        OwnerCheckAccessResult::Controller(self.controller.check_access(&ControllerCheckAccess { id: *id, resource_id, access, password: *password }))
    }

    pub fn unallow_whitelist(
        &mut self,
        OwnerUnallow {
            id, password, resource_id, access
        }: OwnerUnallow<'_, AS::Id, AS::Password, CS::ResourceId, WS::Access>
    ) -> OwnerUnallowWhitelistResult {
        let authentication_result = self.authenticator.authenticate(&Authentication { id, password });
        
        if authentication_result.ok() {
            return OwnerUnallowWhitelistResult::Controller(self.controller.unallow_whitelist(&ControllerUnallow { id, resource_id, access }))
        }

        OwnerUnallowWhitelistResult::Denied(authentication_result)
    }

    pub fn unallow_blacklist(
        &mut self,
        OwnerUnallow {
            id, password, resource_id, access
        }: OwnerUnallow<'_, AS::Id, AS::Password, CS::ResourceId, WS::Access>
    ) -> OwnerUnallowBlacklistResult {
        let authentication_result = self.authenticator.authenticate(&Authentication { id, password });
        
        if authentication_result.ok() {
            return OwnerUnallowBlacklistResult::Controller(self.controller.unallow_blacklist(&ControllerUnallow { id, resource_id, access }))
        }

        OwnerUnallowBlacklistResult::Denied(authentication_result)
    }

    pub fn release_resource_all<'a>(
        &mut self,
        OwnerReleaseResourceAll {
            id, password,
            inputs
        }: OwnerReleaseResourceAll<'a, AS::Id, AS::Password, CS::ResourceId>
    ) -> OwnerReleaseResourceAllResult<'a, CS::Id, CS::ResourceId> {
        let authentication_result = self.authenticator.authenticate(&Authentication { id, password });

        if authentication_result.ok() {
            let controller_inputs = inputs
                .into_iter()
                .map(|resource_id| ControllerReleaseResource { id, resource_id });

            return OwnerReleaseResourceAllResult::Controller(self.controller.release_resource_all(controller_inputs.collect()))
        }

        OwnerReleaseResourceAllResult::Denied(authentication_result)
    }
}