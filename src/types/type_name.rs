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
use wit_parser::Type as WitType;

use super::{Type, TypeMap};

/// Represents the name of a Scala type
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

/// Represents the name of a concrete Scala type
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

/// Represents the name of a Scala type-constructor
#[derive(Clone)]
pub struct Constructor {
    /// The name of the type-constructor
    name: String,

    /// The params of the type-constructor
    params: Vec<String>,
}

impl Constructor {
    /// Creates a new instance of Constructor
    pub fn new(name: &str, params: Vec<Option<WitType>>, type_map: &TypeMap) -> Result<Self> {
        fn process_param(param: Option<WitType>, type_map: &TypeMap) -> Result<Option<Type>> {
            match param.map(|param| Type::from_wit(param, type_map)) {
                Some(Ok(ty)) => Ok(Some(ty)),
                Some(Err(err)) => Err(err),
                None => Ok(None),
            }
        }

        let params: Result<Vec<Option<Type>>> = params
            .into_iter()
            .map(|param| process_param(param, type_map))
            .collect();

        Ok(Self {
            name: name.to_owned(),
            params: params?
                .iter()
                .map(|param| {
                    param
                        .as_ref()
                        .map_or_else(|| "Unit".to_owned(), Type::to_string)
                })
                .collect(),
        })
    }
}

impl Display for Constructor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = &self.name;
        let params = self.params.join(", ");

        write!(f, "{name}[{params}]")
    }
}
