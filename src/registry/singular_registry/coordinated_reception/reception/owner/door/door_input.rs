pub struct DoorPermitsAccessInput<'a, Item, Password, Access> {
    pub item: &'a Item,
    pub password: Option<&'a Password>,
    pub access: &'a Access
}

pub struct DoorGeneratePasswordInput<'a, Item, Access, Policy> {
    pub item: &'a Item,
    pub access: &'a Access,
    pub policy: &'a Policy
}