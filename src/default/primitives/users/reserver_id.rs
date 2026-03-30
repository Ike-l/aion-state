#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ReserverId {
    label: String
}

impl ReserverId {
    pub fn new<T: Into<String>>(label: T) -> Self {
        Self {
            label: label.into()
        }
    }
}