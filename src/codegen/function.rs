// Copyright 2024 Golem Cloud
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::fmt::Display;

use color_eyre::Result;
use convert_case::{Case, Casing};
use wit_parser::{Function as WitFunction, Results as WitResults, Type as WitType};

use super::Render;
use crate::types::{Type, TypeMap};

/// Represents the name of a function param in Scala
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

/// Represents a function param in Scala
struct Param {
    /// The param name
    name: ParamName,

    /// The Scala type associated to the param
    ty: Type,
}

impl Param {
    // Constructs a `Param` from WIT
    pub fn from_wit(name: String, ty: WitType, type_map: &TypeMap) -> Result<Self> {
        Ok(Self {
            name: ParamName::from(name),
            ty: Type::from_wit(ty, type_map)?,
        })
    }
}

/// Represents a function name in Scala
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

/// Represents a function in Scala
pub struct Function {
    /// The function name
    name: FunctionName,

    /// The params of the function
    params: Vec<Param>,

    /// The outputs of the function
    outs: Vec<Type>,
}

impl Function {
    /// Constructs a `Function` from WIT
    pub fn from_wit(function: WitFunction, type_map: &TypeMap) -> Result<Self> {
        let params: Result<Vec<Param>> = function
            .params
            .into_iter()
            .map(|(name, ty)| Param::from_wit(name, ty, type_map))
            .collect();

        let outs: Result<Vec<Type>> = match function.results {
            WitResults::Named(params) => params
                .iter()
                .map(|(_, ty)| Type::from_wit(*ty, type_map))
                .collect(),
            WitResults::Anon(ty) => Type::from_wit(ty, type_map).map(|ty| vec![ty]),
        };

        Ok(Self {
            name: FunctionName::from(function.name),
            params: params?,
            outs: outs?,
        })
    }
}

impl Render for Function {
    fn render(self) -> Result<String> {
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

        Ok(format!("def {name}({params}): {out}"))
    }
}
