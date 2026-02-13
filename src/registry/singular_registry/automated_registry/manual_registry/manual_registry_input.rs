pub struct ManualRegistryAccessInput<'a, Access, ValueId> {
    pub access: &'a Access,
    pub value_id: &'a ValueId,
}

pub struct ManualRegistryReplacementInput<'a, Access, ValueId, Value> {
    pub access: &'a Access,
    pub value_id: ValueId,
    pub value: Option<Value>
}