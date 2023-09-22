use std::fmt::Display;

use convert_case::{Case, Casing};
use wit_parser::Type as WitType;

use super::{Type, TypeMap};

#[derive(Clone)]
pub enum TypeName {
    Concrete(String),
    Constructor { name: String, params: Vec<String> },
}

impl Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeName::Concrete(name) => write!(f, "{name}"),
            TypeName::Constructor { name, params } => {
                let params = params
                    .into_iter()
                    .map(|param| param.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                write!(f, "{name}[{params}]")
            }
        }
    }
}

impl TypeName {
    pub fn concrete(name: &str) -> Self {
        Self::Concrete(name.to_case(Case::UpperCamel))
    }

    pub fn constructor(name: &str, params: Vec<WitType>, type_map: &TypeMap) -> Self {
        Self::Constructor {
            name: name.to_case(Case::UpperCamel),
            params: params
                .into_iter()
                .map(|param| Type::from_wit(param, type_map).to_string())
                .collect(),
        }
    }
}
