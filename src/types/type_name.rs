use std::fmt::Display;

use convert_case::{Case, Casing};

#[derive(Clone)]
pub struct TypeName(String);

impl Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for TypeName {
    fn from(name: String) -> Self {
        Self(name.to_case(Case::UpperCamel))
    }
}
