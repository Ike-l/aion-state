use std::any::TypeId;

pub enum ResourceId {
    Type(TypeId),
    Label(String)
}