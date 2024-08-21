//! You can run this test suite with:
//!
//!     cargo test --test all
//!
//! An argument can be passed as well to filter, based on filename, which test
//! to run
//!
//!     cargo test --test all foo.wit

use anyhow::{bail, Context, Result};
use golem_scalajs_wit_bindgen::generator::generate;
use libtest_mimic::{Arguments, Trial};
use pretty_assertions::StrComparison;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::str;

fn main() {
    env_logger::init();

    let trials: Vec<_> = find_tests()
        .into_iter()
        .map(|test| {
            Trial::test(format!("{test:?}"), move || {
                Runner {}
                    .run(&test)
                    .context(format!("test {:?} failed", test))
                    .map_err(|e| format!("{e:?}").into())
            })
        })
        .collect();

    let args = Arguments::from_args();
    let args = if cfg!(target_family = "wasm") && !cfg!(target_feature = "atomics") {
        Arguments {
            test_threads: Some(1),
            ..args
        }
    } else {
        args
    };

    libtest_mimic::run(&args, trials).exit();
}

fn find_tests() -> Vec<PathBuf> {
    fn find_tests_in_dir(path: &Path) -> Vec<PathBuf> {
        path.read_dir()
            .unwrap()
            .filter_map(|entry| {
                let entry = entry.unwrap();
                let path = entry.path();

                if entry.file_type().unwrap().is_dir() {
                    Some(find_tests_in_dir(&path)) // Recurse into subdirectories
                } else if path.extension().and_then(|s| s.to_str()) == Some("wit") {
                    Some(vec![path]) // Only return .wit files
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }

    let mut tests = Vec::new();
    let root = "tests/wit".as_ref();
    tests.extend(find_tests_in_dir(root));
    tests.sort();
    tests
}

struct Runner {}

impl Runner {
    fn run(&mut self, test: &Path) -> Result<()> {
        let result = generate(test, "tests");
        let result: String = match result {
            Ok(s) => s,
            Err(e) => bail!("{:?}", e),
        };

        self.assert_equals(test, &result, "scala")
    }

    fn assert_equals(
        &mut self,
        test: &Path,
        result: &str,
        extension: &str,
    ) -> Result<(), anyhow::Error> {
        let result_file = test.with_extension(extension);
        if env::var_os("BLESS").is_some() {
            fs::write(&result_file, &result).map_err(|e| e.into())
        } else {
            let expected = fs::read_to_string(&result_file).context(format!(
                "failed to read test expectation file {:?}\nthis can be fixed with BLESS=1",
                result_file
            ))?;
            if expected != result {
                bail!(
                    "failed test: result is not as expected:{}",
                    StrComparison::new(&expected, &result),
                );
            } else {
                Ok(())
            }
        }
    }
}
