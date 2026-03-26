pub struct ManualRegistryAccessInput<'a, ValueId, Access> {
    pub value_id: &'a ValueId,
    pub access: &'a Access,
}

pub struct ManualRegistryReplacementInput<'a, Access, ValueId, Value> {
    pub access: &'a Access,
    pub value_id: ValueId,
    pub value: Option<Value>
}

pub struct ManualRegistryCheckAccess<'a, ValueId, Access> {
    pub value_id: &'a ValueId,
    pub access: &'a Access
}

pub struct ManualRegistryRelease<'a, ValueId, Access> {
    pub value_id: &'a ValueId,
    pub access: &'a Access
}