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

mod primitive;
mod type_map;
mod type_name;

pub use primitive::*;
pub use type_map::*;
pub use type_name::*;

use std::fmt::Display;

use color_eyre::{eyre::eyre, Result};
use wit_parser::Type as WitType;

/// Represents a Scala type
pub enum Type {
    Primitive(Primitive),
    Custom(TypeName),
}

impl Type {
    /// Generates a `Type` from WIT
    pub fn from_wit(ty: WitType, type_map: &TypeMap) -> Result<Self> {
        match ty {
            WitType::Bool => Ok(Type::Primitive(Primitive::Boolean)),
            WitType::U8 | WitType::S8 => Ok(Type::Primitive(Primitive::Byte)),
            WitType::U16 | WitType::S16 => Ok(Type::Primitive(Primitive::Short)),
            WitType::U32 | WitType::S32 => Ok(Type::Primitive(Primitive::Integer)),
            WitType::U64 | WitType::S64 => Ok(Type::Primitive(Primitive::Long)),
            WitType::Float32 => Ok(Type::Primitive(Primitive::Float)),
            WitType::Float64 => Ok(Type::Primitive(Primitive::Double)),
            WitType::Char => Ok(Type::Primitive(Primitive::Char)),
            WitType::String => Ok(Type::Primitive(Primitive::String)),
            WitType::Id(id) => type_map
                .get(&id)
                .map(|ty| Type::Custom(ty.clone()))
                .ok_or(eyre!("Could not find type ID {id:?} in type_map")),
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Primitive(primitive) => write!(f, "{primitive}"),
            Type::Custom(type_name) => write!(f, "{type_name}"),
        }
    }
}
