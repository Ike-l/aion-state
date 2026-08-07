use std::any::type_name;

use serde::Serialize;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct ResourceId {
    label: String,
}

impl Serialize for ResourceId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        serializer.serialize_str(&self.label)
    }
}

impl<'de> serde::Deserialize<'de> for ResourceId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let label = String::deserialize(deserializer)?;
        Ok(Self { label })
    }
}

impl ResourceId {
    pub fn new_label<T: Into<String>>(label: T) -> Self {
        Self { 
            label: label.into()
        }
    }

    pub fn new_type<T: 'static>() -> Self {
        Self {
            label: type_name::<T>().to_owned()
        }
    }
}