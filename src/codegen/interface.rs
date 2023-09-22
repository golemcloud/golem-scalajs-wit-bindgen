use std::fmt::Display;

use convert_case::{Case, Casing};

use id_arena::{Arena, Id};
use wit_parser::{Interface as WitInterface, TypeDef, TypeDefKind, TypeOwner};

use crate::types::TypeMap;

use super::{Function, Record, Variant};

#[derive(Clone)]
struct InterfaceName(String);

impl Display for InterfaceName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for InterfaceName {
    fn from(name: String) -> Self {
        Self(name.to_case(Case::UpperCamel))
    }
}

pub struct Interface {
    name: InterfaceName,
    records: Vec<Record>,
    variants: Vec<Variant>,
    functions: Vec<Function>,
}

impl Interface {
    pub fn from_wit(
        interface_id: Id<WitInterface>,
        interface: &WitInterface,
        types: &Arena<TypeDef>,
        type_map: TypeMap,
    ) -> Self {
        let records = types
            .iter()
            .filter(|(_, ty)| match ty.owner {
                TypeOwner::Interface(id) => id == interface_id,
                _ => false,
            })
            .filter_map(|(_, ty)| match &ty.kind {
                TypeDefKind::Record(record) => Some(Record::from_wit(
                    ty.name.as_ref().unwrap().as_str(),
                    record,
                    &type_map,
                )),
                _ => None,
            })
            .collect();

        let variants = types
            .iter()
            .filter(|(_, ty)| match ty.owner {
                TypeOwner::Interface(id) => id == interface_id,
                _ => false,
            })
            .filter_map(|(_, ty)| match &ty.kind {
                TypeDefKind::Variant(variant) => Some(Variant::from_wit(
                    ty.name.as_ref().unwrap().as_str(),
                    variant,
                    &type_map,
                )),
                _ => None,
            })
            .collect();

        let functions = interface
            .functions
            .iter()
            .map(|(_, function)| Function::from_wit(function.clone(), &type_map))
            .collect();

        Self {
            name: InterfaceName::from(interface.name.clone().unwrap()),
            records,
            variants,
            functions,
        }
    }

    pub fn render(self, package: &str) -> String {
        let records = self
            .records
            .into_iter()
            .map(|record| record.render())
            .collect::<Vec<_>>()
            .join("\n");

        let variants = self
            .variants
            .into_iter()
            .map(|variant| variant.render())
            .collect::<Vec<_>>()
            .join("\n");

        let functions = self
            .functions
            .into_iter()
            .map(|function| function.render())
            .collect::<Vec<_>>()
            .join("\n");

        let name = self.name;

        format!(
            "
                package {package}

                import scala.scalajs.js

                {records}

                {variants}

                trait {name} {{
                    {functions}
                }}
            "
        )
    }
}
