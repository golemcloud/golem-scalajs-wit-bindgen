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
use wit_parser::{Case as WitCase, Variant as WitVariant};

use super::Render;
use crate::types::{ConcreteName, Type, TypeMap, TypeName};

/// Represents the name of a variant case in Scala
struct CaseName(String);

impl Display for CaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for CaseName {
    fn from(name: String) -> Self {
        Self(name.to_case(Case::Camel))
    }
}

/// Represents a variant case in Scala
struct VariantCase {
    /// The case name
    name: CaseName,

    /// The internal case type
    ty: Option<Type>,
}

impl VariantCase {
    /// Constructs a `VariantCase` from WIT
    pub fn from_wit(case: WitCase, type_map: &TypeMap) -> Result<Self> {
        Ok(Self {
            name: CaseName::from(case.name),
            ty: match case.ty {
                Some(ty) => Some(Type::from_wit(ty, type_map)?),
                None => None,
            },
        })
    }
}

/// Represents a variant in Scala
pub struct Variant {
    /// The variant name
    name: TypeName,

    /// The variant cases
    cases: Vec<VariantCase>,
}

impl Variant {
    /// Constructs a `Variant` from WIT
    pub fn from_wit(name: &str, variant: &WitVariant, type_map: &TypeMap) -> Result<Self> {
        let cases: Result<Vec<VariantCase>> = variant
            .cases
            .iter()
            .map(|case| VariantCase::from_wit(case.clone(), type_map))
            .collect();

        Ok(Self {
            name: TypeName::Concrete(ConcreteName::from(name.to_owned())),
            cases: cases?,
        })
    }
}

impl Render for Variant {
    fn render(self) -> Result<String> {
        let name = self.name;

        let constructors = self
            .cases
            .iter()
            .map(
                |VariantCase {
                     name: case_name,
                     ty,
                 }| {
                    let (def_or_val, param_list, val, type_tag) = if let Some(ty) = ty {
                        (
                            "def",
                            format!("(value: {ty})"),
                            "override val `val`: js.UndefOr[Type] = value",
                            format!("type Type = {ty}"),
                        )
                    } else {
                        ("val", String::new(), "", "type Type = Nothing".to_owned())
                    };

                    format!(
                        "
                            {def_or_val} {case_name}{param_list} = new {name} {{
                                {type_tag}
                                
                                val tag: String = \"{case_name}\"
                                {val}
                            }}
                        "
                    )
                },
            )
            .collect::<Vec<_>>()
            .join("\n");

        Ok(format!(
            "
                sealed trait {name} extends js.Object {{ self =>
                    type Type

                    val tag: String
                    val `val`: js.UndefOr[Type]
                }}

                object {name} {{
                    {constructors}
                }}
            "
        ))
    }
}
