pub enum AccessPermission {
    Ok(bool),
    NoCurrentAccess
}

impl AccessPermission {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Ok(true))
    }
}

pub enum RecordAccessResult {
    Merged,
    Inserted
}

pub enum RemoveAccessResult {
    Split,
    NoCurrentAccess
}