pub struct ManualRegistryAccessInput<'a, ValueId, Access> {
    pub value_id: &'a ValueId,
    pub access: &'a Access,
}

pub struct ManualRegistryAcquireViewInput<'a, ValueId, Access> {
    pub value_ids: &'a [&'a ValueId],
    pub access: &'a Access,
}

pub struct ManualRegistryReplacementInput<'a, Access, ValueId, Value> {
    pub access: &'a Access,
    pub value_id: ValueId,
    pub value: Option<Value>
}
