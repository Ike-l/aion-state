pub enum AccessesCheckAccessResult {
    Ok(bool),
    NoCurrentAccess
}

impl AccessesCheckAccessResult {
    pub fn ok(&self) -> bool {
        matches!(self, Self::Ok(true))
    }
}

pub enum AccessesRecordAccessResult {
    Merged,
    Inserted
}

pub enum AccessesReleaseResult {
    Split,
    NoCurrentAccess
}

pub enum AccessesDrainResult<T> {
    Drain(T)
}