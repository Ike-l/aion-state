use std::any::TypeId;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum ResourceId {
    Type(TypeId),
    Label(String)
}

impl ResourceId {
    pub fn new_label<T: Into<String>>(label: T) -> Self {
        Self::Label(label.into())
    }

    pub fn new_type<T: 'static>() -> Self {
        Self::Type(TypeId::of::<T>())
    }
}