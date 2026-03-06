pub struct DoorPermitsAccessInput<'a, ValueId, ValuePassword, Access> {
    pub value_id: &'a ValueId,
    pub value_password: Option<&'a ValuePassword>,
    pub access: &'a Access
}

pub struct DoorGeneratePasswordInput<'a, ValueId, Access, Policy> {
    pub value_id: &'a ValueId,
    pub access: &'a Access,
    pub policy: &'a Policy
}