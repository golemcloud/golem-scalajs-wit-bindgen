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

/// Module containing all the logic for Scala.js code generation
mod codegen;
mod types;

pub mod generator {
    use crate::codegen::Interface;
    use color_eyre::{eyre::eyre, Section};
    use std::path::Path;
    use wit_parser::SourceMap;

    pub fn generate(wit: &Path, package: &str) -> color_eyre::Result<String> {
        let mut source = SourceMap::new();
        source
            .push_file(wit)
            .map_err(|e| eyre!("{e:?}"))
            .with_suggestion(|| "Provide a WIT file that actually exists")?;

        let unresolved_package = source
            .parse()
            .map(|g| g.main)
            .map_err(|e| eyre!("{e:?}"))
            .with_suggestion(|| "Make sure the provided WIT file is valid")?;

        Interface::from_wit(&unresolved_package, "api")?.render(package)
    }
}
