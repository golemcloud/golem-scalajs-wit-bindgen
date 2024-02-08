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

use std::path::Path;

use clap::Parser;
use color_eyre::{eyre::eyre, Result, Section};
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

    let unresolved_package = source
        .parse()
        .map_err(|e| eyre!("{e:?}"))
        .with_suggestion(|| "Make sure the provided WIT file is valid")?;

    Ok(println!(
        "{}",
        Interface::from_wit(&unresolved_package)?.render(&cli_args.package)?
    ))
}
