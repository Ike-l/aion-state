pub enum AccessesCheckAccessResult {
    Ok(bool),
    NoCurrentAccess
}

impl AccessesCheckAccessResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::Ok(true) | 
            Self::NoCurrentAccess => true,
            _ => false
        }
    }
}

pub enum AccessesRecordAccessResult {
    Merged,
    Inserted
}

impl AccessesRecordAccessResult {
    pub fn ok(&self) -> bool {
        true
    }
}

pub enum AccessesReleaseResult {
    Split,
    NoCurrentAccess
}

impl AccessesReleaseResult {
    pub fn ok(&self) -> bool {
        match self {
            Self::Split => true,
            _ => false
        }
    }
}

pub enum AccessesDrainResult<T> {
    Drain(T)
}