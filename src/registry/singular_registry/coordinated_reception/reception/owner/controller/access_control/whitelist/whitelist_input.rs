pub struct WhitelistAccess<'a, Id, Access> {
    pub id: &'a Id,
    pub access: &'a Access
} 

pub struct WhitelistAllow<Id, Access> {
    pub id: Id,
    pub access: Access
}

pub struct WhitelistRelease<'a, Id> {
    pub id: &'a Id
}

pub struct WhitelistBlock<'a, Id, Access> {
    pub id: &'a Id,
    pub access: &'a Access
}