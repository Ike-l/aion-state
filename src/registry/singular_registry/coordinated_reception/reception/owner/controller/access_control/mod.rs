use crate::prelude::{AccessControlAccess, AccessControlAccessResult, Blacklist, BlacklistAccess, BlacklistStorage, Whitelist, WhitelistAccess, WhitelistStorage};

pub mod whitelist;
pub mod blacklist;

pub mod access_control_input;
pub mod access_control_result;

pub struct AccessControl<WS, BS> {
    whitelist: Whitelist<WS>,
    blacklist: Blacklist<BS>,
}

impl<
    WS: WhitelistStorage,
    BS: BlacklistStorage<Id = WS::Id, Access = WS::Access>
> AccessControl<WS, BS> {
    pub fn access(
        &self,
        AccessControlAccess {
            id, access, password
        }: AccessControlAccess<'_, WS::Id, WS::Access, BS::Password>
    ) -> AccessControlAccessResult {
        match password {
            Some(password) => AccessControlAccessResult::Blacklist(self.blacklist.access(BlacklistAccess { id, access, password })),
            None => AccessControlAccessResult::Whitelist(self.whitelist.access(WhitelistAccess { id, access })),
        }
    }
}