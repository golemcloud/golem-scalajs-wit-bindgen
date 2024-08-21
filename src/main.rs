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

use clap::Parser;
use color_eyre::Result;
use golem_scalajs_wit_bindgen::generator;
use std::path::Path;

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

    generator::generate(Path::new(&cli_args.wit), &cli_args.package)
        .map(|code| println!("{}", code))
}
