use crate::prelude::{Deaccessor, sync::Arc};

pub struct DeaccessingResult<AccessResult, D: Deaccessor + ?Sized> {
    raw: Option<AccessResult>,
    used: bool,
    deaccessor: Arc<D>,
    release_input: D::ReleaseInput
}

impl<AccessResult, D: Deaccessor> DeaccessingResult<AccessResult, D> {
    pub fn new(
        access_result: AccessResult, 
        deaccessor: Arc<D>,
        release_input: D::ReleaseInput
    ) -> Self {
        Self { 
            raw: Some(access_result),
            used: false,
            deaccessor,
            release_input
        }
    }

    pub fn update<NewAccessResult>(
        mut self, 
        f: impl FnOnce(AccessResult) -> DeaccessingResult<NewAccessResult, D>
    ) -> DeaccessingResult<NewAccessResult, D> {
        self.used = true;
        f(self.raw.take().unwrap())
    }

    pub fn as_ref(&self) -> Option<&AccessResult> {
        self.raw.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut AccessResult> {
        self.raw.as_mut()
    }
}

impl<AccessResult, D: Deaccessor + ?Sized> Drop for DeaccessingResult<AccessResult, D> {
    fn drop(&mut self) {
        if !self.used {
            self.deaccessor.release_access(&self.release_input);
        }
    }
}