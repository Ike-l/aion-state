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

#[derive(Debug)]
pub enum AccessesRecordAccessResult {
    Merged,
    Inserted
}

impl AccessesRecordAccessResult {
    pub fn ok(&self) -> bool {
        true
    }
}

#[derive(Debug)]
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