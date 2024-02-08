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

use color_eyre::{eyre::eyre, Result};
use convert_case::{Case, Casing};
use id_arena::Id;
use wit_parser::{Interface as WitInterface, TypeDefKind, TypeOwner, UnresolvedPackage};

use super::{Function, Record, Render, Variant};
use crate::types::TypeMap;

/// Represents the name of an interface (trait) in Scala
#[derive(Clone)]
struct InterfaceName(String);

impl Display for InterfaceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&String> for InterfaceName {
    fn from(name: &String) -> Self {
        Self(name.to_case(Case::UpperCamel))
    }
}

/// Represents an interface (trait) in Scala
pub struct Interface {
    /// The name of the interface
    name: InterfaceName,

    /// The records associated to the interface
    records: Vec<Record>,

    /// The variants associated to the interface
    variants: Vec<Variant>,

    /// The functions associated to the interface
    functions: Vec<Function>,
}

impl Interface {
    /// Constructs an `Interface` from WIT
    pub fn from_wit(unresolved_package: &UnresolvedPackage) -> Result<Self> {
        let (interface_id, interface) = Self::get_interface("api", unresolved_package)?;
        let type_map = TypeMap::from(unresolved_package);
        let types = &unresolved_package.types;

        let records: Result<Vec<Record>> = types
            .iter()
            .filter(|(_, ty)| match ty.owner {
                TypeOwner::Interface(id) => id == interface_id,
                _ => false,
            })
            .filter_map(|(_, ty)| match &ty.kind {
                TypeDefKind::Record(record) => ty
                    .name
                    .as_ref()
                    .map(|name| Record::from_wit(name, record, &type_map)),
                _ => None,
            })
            .collect();

        let variants: Result<Vec<Variant>> = types
            .iter()
            .filter(|(_, ty)| match ty.owner {
                TypeOwner::Interface(id) => id == interface_id,
                _ => false,
            })
            .filter_map(|(_, ty)| match &ty.kind {
                TypeDefKind::Variant(variant) => ty
                    .name
                    .as_ref()
                    .map(|name| Variant::from_wit(name, variant, &type_map)),
                _ => None,
            })
            .collect();

        let functions: Result<Vec<Function>> = interface
            .functions
            .iter()
            .map(|(_, function)| Function::from_wit(function.clone(), &type_map))
            .collect();

        Ok(Self {
            name: InterfaceName::from(interface.name.as_ref().ok_or(eyre!(
                "Interface with ID {interface_id:?} does not have a name"
            ))?),
            records: records?,
            variants: variants?,
            functions: functions?,
        })
    }

    fn get_interface<'a>(
        name: &'static str,
        unresolved_package: &'a UnresolvedPackage,
    ) -> Result<(Id<WitInterface>, &'a WitInterface)> {
        unresolved_package
            .interfaces
            .iter()
            .find(|(_, interface)| interface.name.clone().unwrap_or_default() == name)
            .ok_or(eyre!("Interface {name} not found"))
    }

    /// Renders this to a String
    pub fn render(self, package: &str) -> Result<String> {
        fn render(elements: Vec<impl Render>) -> Result<String> {
            let elements: Result<Vec<String>> = elements.into_iter().map(Render::render).collect();
            Ok(elements?.join("\n"))
        }

        let records = render(self.records)?;
        let variants = render(self.variants)?;
        let functions = render(self.functions)?;
        let name = self.name;

        Ok(format!(
            "
                // Generated by golem-scalajs-wit-bindgen
                package {package}

                import scala.scalajs.js
                import scala.scalajs.js.JSConverters._

                {records}

                {variants}

                trait {name} {{
                    type WitResult[+Ok, +Err] = Ok
                    object WitResult {{
                        def ok[Ok](value: Ok): WitResult[Ok, Nothing] = value

                        def err[Err](value: Err): WitResult[Nothing, Err] = throw js.JavaScriptException(value)

                        val unit: WitResult[Unit, Nothing] = ()
                    }}

                    type WitOption[+A] = js.UndefOr[A]
                    object WitOption {{
                        def some[A](value: A): WitOption[A] = value

                        val none: WitOption[Nothing] = js.undefined

                        def fromOption[A](option: Option[A]) =
                        option match {{
                            case Some(value) => value.asInstanceOf[js.UndefOr[A]]
                            case None        => js.undefined
                        }}
                    }}

                    type WitList[A] = js.Array[A]
                    object WitList {{
                        def fromList[A](list: List[A]): WitList[A] = list.toJSArray
                    }}
                    
                    {functions}
                }}
            "
        ))
    }
}
