use serde::Deserialize;

#[derive(Deserialize)]
struct Manifest {
    #[serde(rename = "bin")]
    binaries: Option<Vec<BinaryDefinition>>,
}

#[derive(Deserialize)]
struct BinaryDefinition {
    path: String,
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
}
