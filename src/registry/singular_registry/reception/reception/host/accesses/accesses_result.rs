pub enum AccessPermission {
    UnknownAccessKey
}

impl AccessPermission {
    pub fn ok(&self) -> bool {
        todo!()
    }
}

pub enum RecordAccessResult {
    Merged,
    Inserted
}

pub enum RemoveAccessResult {
    
}