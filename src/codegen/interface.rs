use std::fmt::Display;

use convert_case::{Case, Casing};

use id_arena::Id;
use wit_parser::{Interface as WitInterface, TypeDefKind, TypeOwner, UnresolvedPackage};

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
    pub fn from_wit(unresolved_package: &UnresolvedPackage) -> Self {
        let (interface_id, interface) = Interface::get_interface("api", unresolved_package);
        let type_map = TypeMap::from(unresolved_package);
        let types = &unresolved_package.types;

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

    fn get_interface<'a>(
        name: &'static str,
        unresolved_package: &'a UnresolvedPackage,
    ) -> (Id<WitInterface>, &'a WitInterface) {
        unresolved_package
            .interfaces
            .iter()
            .find(|(_, interface)| interface.name.clone().unwrap_or_default() == name)
            .unwrap()
    }

    pub fn render(self, package: &str) -> String {
        let records = self
            .records
            .into_iter()
            .map(Record::render)
            .collect::<Vec<_>>()
            .join("\n");

        let variants = self
            .variants
            .into_iter()
            .map(Variant::render)
            .collect::<Vec<_>>()
            .join("\n");

        let functions = self
            .functions
            .into_iter()
            .map(Function::render)
            .collect::<Vec<_>>()
            .join("\n");

        let name = self.name;

        format!(
            "
                package {package}

                import scala.scalajs.js

                sealed trait Optional[+A] extends js.Object {{
                    val tag: String
                    val `val`: js.UndefOr[js.Object] = js.undefined
                }}
                object Optional {{
                    def some[A](value: A): Optional[A] = new Optional[A] {{
                        val tag: String                           = \"some\"
                        override val `val`: js.UndefOr[js.Object] = js.Object(value)
                    }}

                    def none: Optional[Nothing] = new Optional[Nothing] {{
                        val tag: String = \"none\"
                    }}
                }}

                sealed trait Result[+Ok, +Err] extends js.Object {{
                    val tag: String
                    val `val`: js.UndefOr[js.Object] = js.undefined
                }}
                object Result {{
                    def ok[Ok](value: Ok): Result[Ok, Nothing] = new Result[Ok, Nothing] {{
                        val tag: String                           = \"ok\"
                        override val `val`: js.UndefOr[js.Object] = js.Object(value)
                    }}

                    def err[Err](value: Err): Result[Nothing, Err] = new Result[Nothing, Err] {{
                        val tag: String                           = \"err\"
                        override val `val`: js.UndefOr[js.Object] = js.Object(value)
                    }}
                }}

                {records}

                {variants}

                trait {name} {{
                    {functions}
                }}
            "
        )
    }
}
