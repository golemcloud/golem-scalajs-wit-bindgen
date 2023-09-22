use std::fmt::Display;

use convert_case::{Case, Casing};

use wit_parser::{Function as WitFunction, Results as WitResults, Type as WitType};

use crate::types::{Type, TypeMap};

struct ParamName(String);

impl Display for ParamName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ParamName {
    fn from(name: String) -> Self {
        Self(name.to_case(Case::Camel))
    }
}

struct Param {
    name: ParamName,
    ty: Type,
}

impl Param {
    pub fn from_wit(name: String, ty: WitType, type_map: &TypeMap) -> Self {
        Self {
            name: ParamName::from(name),
            ty: Type::from_wit(ty, type_map),
        }
    }
}

#[derive(Clone)]
struct FunctionName(String);

impl Display for FunctionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for FunctionName {
    fn from(name: String) -> Self {
        Self(name.to_case(Case::Camel))
    }
}

pub struct Function {
    name: FunctionName,
    params: Vec<Param>,
    outs: Vec<Type>,
}

impl Function {
    pub fn from_wit(function: WitFunction, type_map: &TypeMap) -> Self {
        Self {
            name: FunctionName::from(function.name),
            params: function
                .params
                .into_iter()
                .map(|(name, ty)| Param::from_wit(name, ty, type_map))
                .collect(),
            outs: match function.results {
                WitResults::Named(params) => params
                    .iter()
                    .map(|(_, ty)| Type::from_wit(*ty, type_map))
                    .collect(),
                WitResults::Anon(ty) => vec![Type::from_wit(ty, type_map)],
            },
        }
    }

    pub fn render(self) -> String {
        let params = self
            .params
            .iter()
            .map(|Param { name, ty }| format!("{name}: {ty}"))
            .collect::<Vec<_>>()
            .join(", ");

        let out = {
            let outs = self.outs.iter().map(Type::to_string).collect::<Vec<_>>();

            if outs.is_empty() {
                "Unit".to_owned()
            } else if outs.len() == 1 {
                outs.get(0).unwrap().clone()
            } else {
                format!("({})", outs.join(", "))
            }
        };

        let name = self.name;

        format!("def {name}({params}): {out}")
    }
}
