use std::marker::PhantomData;

use crate::prelude::{Release, sync::Arc};

pub struct ReleasingResult<'a, S, AccessResult, R: Release + ?Sized> {
    raw: Option<AccessResult>,
    consumed: bool,
    releaser: Option<Arc<R>>,
    release_input: Option<R::ReleaseInput>,
    marker: PhantomData<(&'a (), fn() -> S)>,
}

impl<'a, S, AccessResult, R: Release> ReleasingResult<'a, S, AccessResult, R> {
    pub fn new(
        access_result: AccessResult, 
        releaser: Arc<R>,
        release_input: R::ReleaseInput
    ) -> Self {
        Self { 
            raw: Some(access_result),
            consumed: false,
            releaser: Some(releaser),
            release_input: Some(release_input),
            marker: PhantomData,
        }
    }

    pub fn update<NewAccessResult>(
        mut self, 
        f: impl FnOnce(AccessResult) -> NewAccessResult
    ) -> ReleasingResult<'a, S, NewAccessResult, R> {
        self.consumed = true;
        ReleasingResult::<S, NewAccessResult, R>::new(f(self.raw.take().unwrap()), self.releaser.take().unwrap(), self.release_input.take().unwrap())
    }

    pub fn as_ref(&self) -> Option<&AccessResult> {
        self.raw.as_ref()
    }

    pub fn as_mut(&mut self) -> Option<&mut AccessResult> {
        self.raw.as_mut()
    }

    pub fn access_releaser(&self) -> Option<&Arc<R>> {
        self.releaser.as_ref()
    }

    pub fn access_release_input(&self) -> Option<&R::ReleaseInput> {
        self.release_input.as_ref()
    }
}

impl<'a, S, AccessResult, R: Release + ?Sized> Drop for ReleasingResult<'a, S, AccessResult, R> {
    fn drop(&mut self) {
        if !self.consumed {
            self.releaser.take().unwrap().release_access(&self.release_input.take().unwrap());
        }
    }
}
