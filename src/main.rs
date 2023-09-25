use std::{fs, path::Path};

use cargo_scalajs_wit_bindgen::codegen::Interface;
use clap::Parser;
use wit_parser::SourceMap;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct CliArgs {
    /// The path to the input WIT file
    #[arg(short, long)]
    wit: String,

    /// The package of the generated Api.scala file
    #[arg(short, long)]
    package: String,
}

fn main() {
    let cli_args = CliArgs::parse();

    let mut source = SourceMap::new();
    source.push_file(Path::new(&cli_args.wit)).unwrap();
    let unresolved_package = source.parse().unwrap();

    let dest_dir = format!("src/main/scala/{}", cli_args.package.replace(".", "/"));

    fs::create_dir_all(&dest_dir).unwrap();

    fs::write(
        format!("{dest_dir}/Api.scala"),
        Interface::from_wit(&unresolved_package).render(&cli_args.package),
    )
    .unwrap()
}
