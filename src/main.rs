use std::path::Path;

use cargo_scalajs_wit_bindgen::codegen::Interface;
use wit_parser::SourceMap;

fn main() {
    let mut source = SourceMap::new();
    source.push_file(Path::new("main.wit")).unwrap();
    let unresolved_package = source.parse().unwrap();

    // println!(
    //     "{:#?}",
    //     unresolved_package
    //         .types
    //         .iter()
    //         //.map(|(_, ty)| ty)
    //         .collect::<Vec<_>>()
    // );

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
        Interface::from_wit(&unresolved_package).render("example")
    );
}
