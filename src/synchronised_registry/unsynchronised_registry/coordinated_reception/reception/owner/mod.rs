use crate::prelude::{AuthenticateRegister, AuthenticateUnregister, AuthenticateUpdatePassword, Authentication, Authenticator, BlacklistStorage, ControlStorage, Controller, ControllerAllow, ControllerCheckAccess, ControllerIsOwned, ControllerOwn, ControllerReleaseId, ControllerReleaseResource, ControllerUnallow, CredentialStorage, OwnerAllow, OwnerAuthenticate, OwnerAuthenticationResult, OwnerBlacklistAllowResult, OwnerBlacklistUnallowResult, OwnerCheckAccess, OwnerCheckAccessResult, OwnerIsOwned, OwnerOwn, OwnerOwnResult, OwnerRegister, OwnerRegisterResult, OwnerReleaseResource, OwnerReleaseResourceAll, OwnerReleaseResourceAllResult, OwnerReleaseResourceResult, OwnerUnallow, OwnerUnregister, OwnerUnregisterResult, OwnerUpdatePassword, OwnerUpdatePasswordResult, OwnerWhitelistAllowResult, OwnerWhitelistUnallowResult, WhitelistStorage, trace_function};

pub mod authenticator;
pub mod controller;

pub mod owner_result;
pub mod owner_input;

/// Applies `authenticator` semantics and then `controller` semantics
#[derive(Default, serde::Serialize, serde::Deserialize)]
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
                return OwnerUnregisterResult::AuthenticatorUnregisterFailure
            }
        }

        OwnerUnregisterResult::VerificationFailure
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

        OwnerUpdatePasswordResult::Denied
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

        OwnerOwnResult::Denied
    }

    pub fn is_owned(
        &self, 
        OwnerIsOwned {
            resource_id
        }: &OwnerIsOwned<'_, CS::ResourceId>
    ) -> bool {
        trace_function!("Owner Is Owned");

        self.controller.is_owned(&ControllerIsOwned { resource_id })
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

        OwnerReleaseResourceResult::Denied
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

        let authentication_result = self.authenticator.authenticate(&Authentication { id, password });
        
        if authentication_result.ok() {
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
        OwnerCheckAccess {
            user_details, resource_id, access, password
        }: &OwnerCheckAccess<'_, AS::Id, AS::Password, WS::Id, WS::Access, BS::Password>
    ) -> OwnerCheckAccessResult {
        trace_function!("Owner Check Access");

        if let Some((id, id_password)) = user_details {
            let authentication_result = self.authenticator.authenticate(&Authentication { id, password: *id_password });

            if !authentication_result.ok() {
                return OwnerCheckAccessResult::Denied
            }
        }

        let id = user_details.map(|(id, _)| id);
        OwnerCheckAccessResult::Controller(self.controller.check_access(&ControllerCheckAccess { id, resource_id, access, password: *password }))
    }

    pub fn unallow_whitelist(
        &mut self,
        OwnerUnallow {
            id, password, resource_id, access
        }: &OwnerUnallow<'_, AS::Id, AS::Password, CS::ResourceId, WS::Access>
    ) -> OwnerWhitelistUnallowResult {
        let authentication_result = self.authenticator.authenticate(&Authentication { id, password });
        
        if authentication_result.ok() {
            return OwnerWhitelistUnallowResult::Controller(self.controller.unallow_whitelist(&ControllerUnallow { id, resource_id, access }))
        }

        OwnerWhitelistUnallowResult::Denied
    }

    pub fn unallow_blacklist(
        &mut self,
        OwnerUnallow {
            id, password, resource_id, access
        }: &OwnerUnallow<'_, AS::Id, AS::Password, CS::ResourceId, WS::Access>
    ) -> OwnerBlacklistUnallowResult {
        let authentication_result = self.authenticator.authenticate(&Authentication { id, password });
        
        if authentication_result.ok() {
            return OwnerBlacklistUnallowResult::Controller(self.controller.unallow_blacklist(&ControllerUnallow { id, resource_id, access }))
        }

        OwnerBlacklistUnallowResult::Denied
    }

    pub fn release_resource_all<'a>(
        &mut self,
        OwnerReleaseResourceAll {
            id, password,
            inputs
        }: OwnerReleaseResourceAll<'a, AS::Id, AS::Password, CS::ResourceId>
    ) -> OwnerReleaseResourceAllResult {
        let authentication_result = self.authenticator.authenticate(&Authentication { id, password });

        if authentication_result.ok() {
            let controller_inputs = inputs
                .into_iter()
                .map(|resource_id| ControllerReleaseResource { id, resource_id })
                .collect::<Vec<_>>();

            return OwnerReleaseResourceAllResult::Controller(self.controller.release_resource_all(controller_inputs.iter().collect()))
        }

        OwnerReleaseResourceAllResult::Denied
    }

    pub fn authenticate(
        &self,
        OwnerAuthenticate {
            id, password
        }: &OwnerAuthenticate<'_, AS::Id, AS::Password>
    ) -> OwnerAuthenticationResult {
        OwnerAuthenticationResult::Authenticator(self.authenticator.authenticate(&Authentication { id, password }))
    }
}


impl<AS: CredentialStorage, WS, BS, CS> Owner<AS, WS, BS, CS> 
    where AS::Id: Clone
{
    pub fn registered(&self) -> Vec<AS::Id> {
        trace_function!("Owner Registered");

        self.authenticator.registered()
    }
}