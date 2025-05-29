//
// Constants regarding the deployer environment and conventions
//
/// Where executables are moved to in order to persist across deploys, relative to workspace root
pub const EXECUTABLE_DIRNAME: &str = ".klyra-executables";
/// Where general files will persist across deploys, relative to workspace root. Used by plugins.
pub const STORAGE_DIRNAME: &str = ".klyra-storage";

pub const API_URL_LOCAL: &str = "http://localhost:8001";
pub const API_URL_PRODUCTION: &str = "https://api.klyra.rs";
#[cfg(debug_assertions)]
pub const API_URL_DEFAULT: &str = API_URL_LOCAL;
#[cfg(not(debug_assertions))]
pub const API_URL_DEFAULT: &str = API_URL_PRODUCTION;

// Crate names for checking cargo metadata
pub const NEXT_NAME: &str = "klyra-next";
pub const RUNTIME_NAME: &str = "klyra-runtime";

pub mod limits {
    pub const MAX_PROJECTS_DEFAULT: u32 = 3;
    pub const MAX_PROJECTS_EXTRA: u32 = 15;
}
