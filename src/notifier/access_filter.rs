use std::marker::PhantomData;

pub trait AccessFilter {
    type Error;
    
    fn retry(&self, error: &Self::Error) -> bool;
}

pub struct PermissiveFilter<Error>(PhantomData<Error>);

impl<Error> Default for PermissiveFilter<Error> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<Error> AccessFilter for PermissiveFilter<Error> {
    type Error = Error;

    fn retry(&self, _error: &Self::Error) -> bool { true }
}
