pub trait AccessFilter {
    type Error;
    
    fn retry(&self, error: &Self::Error) -> bool;
}