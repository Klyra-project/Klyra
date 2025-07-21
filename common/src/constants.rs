//! Shared constants used across Klyra crates

/// Where executables are moved to in order to persist across deploys, relative to workspace root
pub const EXECUTABLE_DIRNAME: &str = ".klyra-executables";
/// Where general files will persist across deploys, relative to workspace root. Used by plugins.
pub const STORAGE_DIRNAME: &str = ".klyra-storage";

// URLs
pub const API_URL_LOCAL: &str = "http://localhost:8001";
pub const API_URL_PRODUCTION: &str = "https://api.klyra.rs";
pub const API_URL_BETA: &str = "https://api.klyra.dev";
#[cfg(debug_assertions)]
pub const API_URL_DEFAULT: &str = API_URL_LOCAL;
#[cfg(not(debug_assertions))]
pub const API_URL_DEFAULT: &str = API_URL_PRODUCTION;

pub const klyra_STATUS_URL: &str = "https://status.klyra.rs";
pub const klyra_LOGIN_URL: &str = "https://console.klyra.rs/new-project";
pub const klyra_GH_REPO_URL: &str = "https://github.com/klyra-hq/klyra";
pub const klyra_GH_ISSUE_URL: &str = "https://github.com/klyra-hq/klyra/issues/new/choose";
pub const klyra_INSTALL_DOCS_URL: &str = "https://docs.klyra.rs/getting-started/installation";
pub const klyra_IDLE_DOCS_URL: &str = "https://docs.klyra.rs/getting-started/idle-projects";
pub const EXAMPLES_REPO: &str = "https://github.com/klyra-hq/klyra-examples";
pub const EXAMPLES_README: &str =
    "https://github.com/klyra-hq/klyra-examples#how-to-clone-run-and-deploy-an-example";
pub const EXAMPLES_TEMPLATES_TOML: &str =
    "https://raw.githubusercontent.com/klyra-hq/klyra-examples/main/templates.toml";

// Crate name for checking cargo metadata
pub const RUNTIME_NAME: &str = "klyra-runtime";

/// Current version field in requests to provisioner
pub const RESOURCE_SCHEMA_VERSION: u32 = 1;

/// Current version field in `examples/templates.toml`
pub const TEMPLATES_SCHEMA_VERSION: u32 = 1;

/// Timeframe before a project is considered idle
pub const DEFAULT_IDLE_MINUTES: u64 = 30;

/// Function to set [DEFAULT_IDLE_MINUTES] as a serde default
pub const fn default_idle_minutes() -> u64 {
    DEFAULT_IDLE_MINUTES
}

/// The port that deployer tells the runtime to expose its service on
pub const DEPLOYER_SERVICE_HTTP_PORT: u16 = 8000;

pub mod limits {
    pub const MAX_PROJECTS_DEFAULT: u32 = 3;
    pub const MAX_PROJECTS_EXTRA: u32 = 15;
}

pub mod headers {
    use http::HeaderName;

    pub static X_CARGO_klyra_VERSION: HeaderName =
        HeaderName::from_static("x-cargo-klyra-version");
}
