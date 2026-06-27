use crate::prelude::{Releaser, sync::Arc};

pub struct ReleasingResult<AccessResult, R: Releaser + ?Sized> {
    raw: Option<AccessResult>,
    used: bool,
    releaser: Option<Arc<R>>,
    release_input: Option<R::ReleaseInput>
}

impl<AccessResult, R: Releaser> ReleasingResult<AccessResult, R> {
    pub fn new(
        access_result: AccessResult, 
        releaser: Arc<R>,
        release_input: R::ReleaseInput
    ) -> Self {
        Self { 
            raw: Some(access_result),
            used: false,
            releaser: Some(releaser),
            release_input: Some(release_input)
        }
    }

    pub fn update<NewAccessResult>(
        mut self, 
        f: impl FnOnce(AccessResult) -> NewAccessResult
    ) -> ReleasingResult<NewAccessResult, R> {
        self.used = true;
        ReleasingResult::new(f(self.raw.take().unwrap()), self.releaser.take().unwrap(), self.release_input.take().unwrap())
    }

    pub fn as_ref(&self) -> Option<&AccessResult> {
        self.raw.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut AccessResult> {
        self.raw.as_mut()
    }
}

impl<AccessResult, R: Releaser + ?Sized> Drop for ReleasingResult<AccessResult, R> {
    fn drop(&mut self) {
        if !self.used {
            self.releaser.take().unwrap().release_access(&self.release_input.take().unwrap());
        }
    }
}