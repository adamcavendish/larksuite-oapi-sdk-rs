use std::fs;
use std::path::{Component, Path};

use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn manifest<T: DeserializeOwned>(source: &str, name: &str) -> T {
    serde_json::from_str(source).unwrap_or_else(|error| panic!("{name} protocol manifest: {error}"))
}

pub fn fixture(root: &str, path: &str) -> Value {
    let relative = Path::new(path);
    assert!(
        relative.is_relative()
            && !relative
                .components()
                .any(|part| matches!(part, Component::ParentDir)),
        "unsafe fixture path {path}"
    );
    let full_path = Path::new(root).join(relative);
    serde_json::from_str(
        &fs::read_to_string(&full_path)
            .unwrap_or_else(|error| panic!("read fixture {}: {error}", full_path.display())),
    )
    .unwrap_or_else(|error| panic!("parse fixture {}: {error}", full_path.display()))
}
