use std::path::Path;

use id_arena::Id;
use scalajs_wit_bindgen::{
    codegen::{Interface, Record, Variant},
    types::TypeMap,
};
use wit_parser::{Interface as WitInterface, SourceMap, TypeDefKind, TypeOwner, UnresolvedPackage};

fn main() {
    let mut source = SourceMap::new();
    source.push_file(Path::new("main.wit")).unwrap();
    let unresolved_package = source.parse().unwrap();
    let (interface_id, interface) = get_interface("api", &unresolved_package);

    println!(
        "{:#?}",
        unresolved_package
            .types
            .iter()
            //.map(|(_, ty)| ty)
            .collect::<Vec<_>>()
    );

    // println!(
    //     "{:#?}",
    //     unresolved_package
    //         .interfaces
    //         .iter()
    //         .find(|(id, _)| *id == interface_id)
    //         .unwrap()
    // );

    println!(
        "{}",
        Interface::from_wit(
            interface_id,
            interface,
            &unresolved_package.types,
            (&unresolved_package).into()
        )
        .render("example")
    );
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
