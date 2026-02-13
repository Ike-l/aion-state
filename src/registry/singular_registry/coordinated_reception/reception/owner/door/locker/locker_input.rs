pub struct LockerPermitsAccessInput<'a, ValueId> {
    pub value_id: &'a ValueId
}

pub struct LockInput<'a, ValueId> {
    pub value_id: &'a ValueId
}

pub struct UnlockInput<'a, ValueId> {
    pub value_id: &'a ValueId
}