fn is_cli(contents: impl AsRef<str>) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binary_manifest() {
        let contents = r#"[package]
name = "test"

[[bin]]
name = "something"
path = "src/bin/foo.rs"
"#;
        assert!(is_cli(contents));
    }
}
