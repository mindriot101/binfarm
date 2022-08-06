use std::collections::HashMap;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Manifest {
    #[serde(rename = "bin")]
    binaries: Option<Vec<BinaryDefinition>>,

    dependencies: Option<HashMap<String, DependencyDefinition>>,
}

#[derive(Debug, Deserialize)]
struct BinaryDefinition {
    path: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum DependencyDefinition {
    VersionOnly(String),
    Complex(HashMap<String, String>),
}

fn is_cli_from_manifest(manifest: Manifest, files: &[&str]) -> bool {
    // explicit binaries
    if let Some(bin_definitions) = manifest.binaries {
        return !bin_definitions.is_empty();
    }

    // source directory has main.rs
    if files.contains(&"src/main.rs") {
        return true;
    }

    // source directory has bin subdir
    for path in files {
        if path.starts_with("src/bin/") {
            return true;
        }
    }

    // common crates
    if let Some(dependencies) = manifest.dependencies {
        for dependency in &["clap", "structopt"] {
            if dependencies.contains_key(*dependency) {
                return true;
            }
        }
    }

    false
}

fn is_cli(contents: impl AsRef<str>, files: &[&str]) -> bool {
    let manifest: Manifest =
        toml::from_str(contents.as_ref()).expect("invalid manifest - could not parse toml");
    is_cli_from_manifest(manifest, files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_binary() {
        let contents = r#"[package]
name = "test"

[[bin]]
name = "something"
path = "src/bin/foo.rs"
"#;

        assert!(is_cli(contents, &[]));
    }

    #[test]
    fn non_binary() {
        let contents = r#"[package]
name = "test"
"#;

        assert!(!is_cli(contents, &[]));
    }

    #[test]
    fn has_main_rs() {
        let contents = r#"[package]
name = "test"
"#;

        assert!(is_cli(contents, &["src/main.rs"]));
    }

    #[test]
    fn has_bin_subfolder() {
        let contents = r#"[package]
name = "test"
"#;

        assert!(is_cli(contents, &["src/lib.rs", "src/bin/foo.rs"]));
    }

    #[test]
    fn has_cli_crates() {
        let examples = &[
            r#"[package]
            name = "test"

            [dependencies]
            clap = "0.1.0"
            "#,
            r#"[package]
            name = "test"

            [dependencies]
            clap = { version = "0.1.0" }
            "#,
            r#"[package]
            name = "test"

            [dependencies]
            structopt = "*"
            "#,
        ];
        for contents in examples {
            assert!(is_cli(contents, &[]));
        }
    }
}
