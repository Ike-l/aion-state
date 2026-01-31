pub struct ManualRegistryAccessInput<'a, Access, Key> {
    pub access: &'a Access,
    pub key: &'a Key,
}

pub struct ManualRegistryReplacementInput<'a, Access, Key, Value> {
    pub access: &'a Access,
    pub key: Key,
    pub value: Option<Value>
}