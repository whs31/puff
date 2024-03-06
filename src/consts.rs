pub const POPPY_NAME: &str = "poppy";
pub const POPPY_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const POPPY_MANIFEST_NAMES: [&str; 3] = [
  "poppy-manifest.toml",
  "poppy.toml",
  "pppm.toml"
];
#[allow(dead_code)] pub const POPPY_EXTENSIONS_DIRECTORY_NAME: &str = ".poppy";

/*
    ├── registry/
    │   └── .git ...                - moved to .local/share/poppy/registry
    ├── cache/
    │   └── downloaded libs here... - moved to .local/share/poppy/blobs

    .poppy/
    ├── todo: recipes, etc.
    target/
    └── dependencies/
        └── installed libs here...
 */

#[allow(dead_code)] pub const POPPY_TARGET_DIRECTORY_NAME: &str = "target";
pub const POPPY_REGISTRY_DIRECTORY_NAME: &str = "registry";
pub const POPPY_CACHE_DIRECTORY_NAME: &str = "blobs";
pub const POPPY_INSTALLATION_DIRECTORY_NAME: &str = "dependencies";