//! Shared constants used across Klyra crates

/// Used by plugins for local file storage.
pub const STORAGE_DIRNAME: &str = ".klyra-storage";

// URLs
pub const API_URL_DEFAULT_BETA: &str = "https://api.klyra.dev";
pub const klyra_CONSOLE_URL: &str = "https://console.klyra.dev";

pub const klyra_INSTALL_DOCS_URL: &str = "https://docs.klyra.dev/getting-started/installation";

pub const klyra_GH_REPO_URL: &str = "https://github.com/klyra-hq/klyra";
pub const klyra_GH_ISSUE_URL: &str = "https://github.com/klyra-hq/klyra/issues/new/choose";
pub const EXAMPLES_REPO: &str = "https://github.com/klyra-hq/klyra-examples";
pub const EXAMPLES_README: &str =
    "https://github.com/klyra-hq/klyra-examples#how-to-clone-run-and-deploy-an-example";
pub const EXAMPLES_TEMPLATES_TOML: &str =
    "https://raw.githubusercontent.com/klyra-hq/klyra-examples/main/templates.toml";

/// Crate name for checking cargo metadata
pub const RUNTIME_NAME: &str = "klyra-runtime";

/// Current version field in `examples/templates.toml`
pub const TEMPLATES_SCHEMA_VERSION: u32 = 1;

pub mod headers {
    use http::HeaderName;

    pub static X_CARGO_klyra_VERSION: HeaderName =
        HeaderName::from_static("x-cargo-klyra-version");
}
