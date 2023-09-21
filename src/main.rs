use std::path::Path;

use convert_case::{Case, Casing};
use wit_parser::{Record, SourceMap, Type, TypeDefKind, TypeOwner};

fn main() {
    let mut source = SourceMap::new();
    source.push_file(Path::new("main.wit")).unwrap();
    let unresolved_package = source.parse().unwrap();
    let interface_id = unresolved_package
        .interfaces
        .iter()
        .find(|(_, interface)| interface.name.clone().unwrap_or_else(|| "".to_owned()) == "api")
        .map(|(id, _)| id)
        .unwrap();

    println!(
        "{:#?}",
        unresolved_package
            .interfaces
            .iter()
            .map(|(_, interface)| interface)
            .find(|interface| interface.name.clone().unwrap_or_else(|| "".to_owned()) == "api")
            .unwrap()
    );

    unresolved_package
        .types
        .iter()
        .filter(|(_, tpe)| match tpe.owner {
            TypeOwner::Interface(id) => id == interface_id,
            _ => false,
        })
        .map(|(_, tpe)| match &tpe.kind {
            TypeDefKind::Record(record) => {
                generate_scalajs_record(tpe.name.as_ref().unwrap(), record)
            }
            _ => "".to_owned(),
        })
        .for_each(|code| println!("{code}"));
}

fn generate_scalajs_record(name: &str, record: &Record) -> String {
    let name = name.to_case(Case::UpperCamel);

    let fields_vec = record
        .clone()
        .fields
        .into_iter()
        .map(|field| {
            (
                field.name.to_case(Case::Camel),
                generate_scala_type(field.ty).unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let fields = fields_vec
        .iter()
        .map(|(name, ty)| format!("val {name}: {ty}"))
        .collect::<Vec<_>>()
        .join("\n");

    let apply_params = fields_vec
        .iter()
        .map(|(name, ty)| format!("{name}: {ty}"))
        .collect::<Vec<_>>()
        .join(", ");

    let apply_temp_vars = fields_vec
        .iter()
        .map(|(name, ty)| format!("val {name}0: {ty} = {name}"))
        .collect::<Vec<_>>()
        .join("\n");

    let new_vars = fields_vec
        .iter()
        .map(|(name, ty)| format!("val {name}: {ty} = {name}0"))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "
            sealed trait {name} extends js.Object {{
                {fields}
            }}
            object {name} {{
                def apply({apply_params}): {name} = {{
                    {apply_temp_vars}

                    new {name} {{
                        {new_vars}
                    }}
                }}
            }}
        "
    )
}

fn generate_scala_type(ty: Type) -> Option<String> {
    match ty {
        Type::Bool => Some("Boolean".to_owned()),
        Type::U8 => Some("Byte".to_owned()),
        Type::U16 => Some("Short".to_owned()),
        Type::U32 => Some("Integer".to_owned()),
        Type::U64 => Some("Long".to_owned()),
        Type::S8 => Some("Byte".to_owned()),
        Type::S16 => Some("Short".to_owned()),
        Type::S32 => Some("Integer".to_owned()),
        Type::S64 => Some("Long".to_owned()),
        Type::Float32 => Some("Float".to_owned()),
        Type::Float64 => Some("Double".to_owned()),
        Type::Char => Some("Char".to_owned()),
        Type::String => Some("String".to_owned()),
        Type::Id(_) => None,
    }
}
