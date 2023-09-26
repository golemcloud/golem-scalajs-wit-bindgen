use std::{fs, path::Path};

use clap::Parser;
use color_eyre::{eyre::eyre, Result, Section};
use color_print::cprintln;
use golem_scalajs_wit_bindgen::codegen::Interface;
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

fn main() -> Result<()> {
    color_eyre::install()?;

    let cli_args = CliArgs::parse();

    let mut source = SourceMap::new();
    source
        .push_file(Path::new(&cli_args.wit))
        .map_err(|e| eyre!("{e:?}"))
        .with_suggestion(|| "Provide a WIT file that actually exists")?;

    let unresolved_package = source.parse().map_err(|e| eyre!("{e:?}"))?;

    let dest_dir = format!("src/main/scala/{}", cli_args.package.replace('.', "/"));

    fs::create_dir_all(&dest_dir)?;

    fs::write(
        format!("{dest_dir}/Api.scala"),
        Interface::from_wit(&unresolved_package)?.render(&cli_args.package)?,
    )?;

    Ok(cprintln!("<green>WIT file generated successfully</green>"))
}
