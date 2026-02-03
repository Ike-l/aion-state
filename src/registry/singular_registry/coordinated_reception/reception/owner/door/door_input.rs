pub struct DoorPermitsAccessInput<'a, Item, Password, Access> {
    pub item: &'a Item,
    pub password: Option<&'a Password>,
    pub access: &'a Access
}
