pub struct WhitelistAccess<'a, Id, Access> {
    pub id: &'a Id,
    pub access: &'a Access
} 

pub struct WhitelistAllow<Id, Access> {
    pub id: Id,
    pub access: Access
}