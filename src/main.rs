use std::path::Path;

use id_arena::Id;
use scalajs_wit_bindgen::{
    codegen::{Record, Variant},
    types::TypeMap,
};
use wit_parser::{Interface, SourceMap, TypeDefKind, TypeOwner, UnresolvedPackage};

fn main() {
    let mut source = SourceMap::new();
    source.push_file(Path::new("main.wit")).unwrap();
    let unresolved_package = source.parse().unwrap();
    let (interface_id, type_map) = get_interface("api", &unresolved_package);

    // println!(
    //     "{:#?}",
    //     unresolved_package
    //         .types
    //         .iter()
    //         //.map(|(_, ty)| ty)
    //         .collect::<Vec<_>>()
    // );

    render(interface_id, type_map, unresolved_package);
}

fn get_interface(name: &str, unresolved_package: &UnresolvedPackage) -> (Id<Interface>, TypeMap) {
    unresolved_package
        .interfaces
        .iter()
        .find(|(_, interface)| interface.name.clone().unwrap_or_default() == name)
        .map(|(id, interface)| (id, TypeMap::from(interface.clone())))
        .unwrap()
}

fn render(interface_id: Id<Interface>, type_map: TypeMap, unresolved_package: UnresolvedPackage) {
    unresolved_package
        .types
        .into_iter()
        .filter(|(_, ty)| match ty.owner {
            TypeOwner::Interface(id) => id == interface_id,
            _ => false,
        })
        .map(|(_, ty)| match &ty.kind {
            TypeDefKind::Record(record) => {
                Record::from_wit(ty.name.as_ref().unwrap().as_str(), record, &type_map).render()
            }
            TypeDefKind::Variant(variant) => {
                Variant::from_wit(ty.name.as_ref().unwrap().as_str(), variant, &type_map).render()
            }
            _ => String::new(),
        })
        .for_each(|code| println!("{code}"));
}
