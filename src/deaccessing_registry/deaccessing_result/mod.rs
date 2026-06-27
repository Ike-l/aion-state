use crate::prelude::RawDeaccessingResult;

pub mod raw_deaccessing_result;

pub struct DeaccessingResult {
    raw: Option<RawDeaccessingResult>
}