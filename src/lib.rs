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

fn is_cli_from_manifest(manifest: Manifest) -> bool {
    if let Some(bin_definitions) = manifest.binaries {
        return bin_definitions.len() > 0;
    }

    false
}

fn is_cli(contents: impl AsRef<str>) -> bool {
    let manifest: Manifest =
        toml::from_str(contents.as_ref()).expect("invalid manifest - could not parse toml");
    is_cli_from_manifest(manifest)
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

        assert!(is_cli(contents));
    }

    #[test]
    fn non_binary() {
        let contents = r#"[package]
name = "test"
"#;

        assert!(!is_cli(contents));
    }
}
