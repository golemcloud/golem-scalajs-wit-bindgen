use std::fmt::Display;

use convert_case::{Case, Casing};
use wit_parser::Type as WitType;

use super::{Type, TypeMap};

#[derive(Clone)]
pub enum TypeName {
    Concrete(ConcreteName),
    Constructor(Constructor),
}

impl Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeName::Concrete(name) => write!(f, "{name}"),
            TypeName::Constructor(constructor) => write!(f, "{constructor}"),
        }
    }
}

#[derive(Clone)]
pub struct ConcreteName(String);

impl From<String> for ConcreteName {
    fn from(name: String) -> Self {
        Self(name.to_case(Case::UpperCamel))
    }
}

impl Display for ConcreteName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone)]
pub struct Constructor {
    name: String,
    params: Vec<String>,
}

impl Constructor {
    pub fn new(name: &str, params: Vec<WitType>, type_map: &TypeMap) -> Self {
        Self {
            name: name.to_case(Case::UpperCamel),
            params: params
                .into_iter()
                .map(|param| Type::from_wit(param, type_map).to_string())
                .collect(),
        }
    }
}

impl Display for Constructor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let params = self.params.join(", ");

        write!(f, "{name}[{params}]")
    }
}
