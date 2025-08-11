use std::{
    fmt::Write,
    fs::File,
    io::stdout,
    path::{Path, PathBuf},
    str::FromStr,
    time::Duration,
};

use anyhow::{bail, Context, Result};
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use clap_mangen::Man;
use futures::StreamExt;
use git2::{Repository, StatusOptions};
use indoc::writedoc;
use klyra_common::{
    constants::{klyra_GH_ISSUE_URL, klyra_GH_REPO_URL, klyra_INSTALL_DOCS_URL},
    semvers_are_compatible,
    templates::TemplatesSchema,
};
use tokio_tungstenite::tungstenite::{self, Message};
use tracing::{debug, trace, warn};

use crate::{Binary, KlyraArgs};

// /// Can be used during testing
// async fn get_templates_schema() -> Result<TemplatesSchema> {
//     Ok(toml::from_str(include_str!(
//         "../../examples/templates.toml"
//     ))?)
// }
pub async fn get_templates_schema() -> Result<TemplatesSchema> {
    let client = reqwest::Client::new();
    Ok(toml::from_str(
        &client
            .get(klyra_common::constants::EXAMPLES_TEMPLATES_TOML)
            .send()
            .await?
            .text()
            .await?,
    )?)
}

pub fn is_dirty(repo: &Repository) -> Result<()> {
    let mut status_options = StatusOptions::new();
    status_options.include_untracked(true);
    let statuses = repo
        .statuses(Some(&mut status_options))
        .context("getting status of repository files")?;

    if !statuses.is_empty() {
        let mut error = format!(
            "{} files in the working directory contain changes that were not yet committed into git:\n",
            statuses.len()
        );

        for status in statuses.iter() {
            trace!(
                path = status.path(),
                status = ?status.status(),
                "found file with updates"
            );

            let rel_path = status.path().context("getting path of changed file")?;

            writeln!(error, "{rel_path}").expect("to append error");
        }

        writeln!(error).expect("to append error");
        writeln!(error, "To proceed despite this and include the uncommitted changes, pass the `--allow-dirty` flag (alias `--ad`)").expect("to append error");

        bail!(error);
    }

    Ok(())
}

pub async fn check_and_warn_runtime_version(path: &Path) -> Result<Option<String>> {
    if let Err(err) = check_version(path).await {
        warn!("{}", err);
        if let Some(mismatch) = err.downcast_ref::<VersionMismatchError>() {
            let mut warning = String::new();
            writeln!(&mut warning, "Warning: {}.", mismatch).unwrap();
            if mismatch.klyra_runtime > mismatch.cargo_klyra {
                // The runtime is newer than cargo-klyra so we
                // should help the user to update cargo-klyra.

                writedoc! {
                    &mut warning,
                    "
                    Hint: A newer version of Klyra CLI is available.
                          Check out the installation docs for how to update: {}
                    ",
                    klyra_INSTALL_DOCS_URL,
                }
                .unwrap();
            } else {
                writedoc! {
                    &mut warning,
                    "
                    Hint: A newer version of klyra-runtime is available.
                          Change its version to {} in Cargo.toml to update it,
                          or run this command: cargo add klyra-runtime@{}
                    ",
                    mismatch.cargo_klyra,
                    mismatch.cargo_klyra,
                }
                .unwrap();
            }
            return Ok(Some(warning));
        } else {
            return Err(err.context(
                format!(
                    "Failed to verify the version of klyra-runtime in {}. Is cargo targeting the correct executable?",
                    path.display()
                )
            ));
        }
    }

    Ok(None)
}

pub async fn check_version(runtime_path: &Path) -> Result<()> {
    debug!(
        "Checking version of runtime binary at {}",
        runtime_path.display()
    );

    // should always be a valid semver
    let my_version = semver::Version::from_str(crate::VERSION).unwrap();

    if !runtime_path.try_exists()? {
        bail!("klyra-runtime binary not found");
    }

    // Get runtime version from klyra-runtime cli
    // It should print the version and exit immediately, so a timeout is used to not launch servers with non-Klyra setups
    let stdout = tokio::time::timeout(Duration::from_millis(3000), async move {
        tokio::process::Command::new(runtime_path)
            .arg("--version")
            .kill_on_drop(true) // if the binary does not halt on its own, not killing it will leak child processes
            .output()
            .await
            .context("Failed to run the klyra-runtime binary to check its version")
            .map(|o| o.stdout)
    })
    .await
    .context("Checking the version of klyra-runtime timed out. Make sure the executable is using #[klyra-runtime::main].")??;

    // Parse the version, splitting the version from the name and
    // and pass it to `to_semver()`.
    let runtime_version = semver::Version::from_str(
        std::str::from_utf8(&stdout)
            .context("klyra-runtime version should be valid utf8")?
            .split_once(' ')
            .context("klyra-runtime version should be in the `name version` format")?
            .1
            .trim(),
    )
    .context("failed to convert user's runtime version to semver")?;

    if semvers_are_compatible(&my_version, &runtime_version) {
        Ok(())
    } else {
        Err(VersionMismatchError {
            klyra_runtime: runtime_version,
            cargo_klyra: my_version,
        })
        .context("klyra-runtime and Klyra CLI have incompatible versions")
    }
}

#[derive(Debug)]
pub struct VersionMismatchError {
    klyra_runtime: semver::Version,
    cargo_klyra: semver::Version,
}

impl std::fmt::Display for VersionMismatchError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "klyra-runtime {} and Klyra CLI {} are incompatible",
            self.klyra_runtime, self.cargo_klyra
        )
    }
}

impl std::error::Error for VersionMismatchError {}

pub fn generate_completions(bin: Binary, shell: Shell, output: Option<PathBuf>) -> Result<()> {
    let name = bin.name();
    let mut app = KlyraArgs::command();
    match output {
        Some(path) => generate(shell, &mut app, name, &mut File::create(path)?),
        None => generate(shell, &mut app, name, &mut stdout()),
    };

    Ok(())
}

/// Prints a combined manpage by concatenating the main page with each subcommand's page
pub fn generate_manpage() -> Result<()> {
    let app = KlyraArgs::command();
    let output = std::io::stdout();
    let mut output_handle = output.lock();

    Man::new(app.clone()).render(&mut output_handle)?;

    for subcommand in app.get_subcommands() {
        let primary = Man::new(subcommand.clone());
        primary.render_name_section(&mut output_handle)?;
        primary.render_synopsis_section(&mut output_handle)?;
        primary.render_description_section(&mut output_handle)?;
        primary.render_options_section(&mut output_handle)?;
        // For example, `generate` has sub-commands `shell` and `manpage`
        if subcommand.has_subcommands() {
            primary.render_subcommands_section(&mut output_handle)?;
            for sb in subcommand.get_subcommands() {
                let secondary = Man::new(sb.clone());
                secondary.render_name_section(&mut output_handle)?;
                secondary.render_synopsis_section(&mut output_handle)?;
                secondary.render_description_section(&mut output_handle)?;
                secondary.render_options_section(&mut output_handle)?;
            }
        }
    }

    Ok(())
}

pub fn open_gh_issue() -> Result<()> {
    let _ = webbrowser::open(klyra_GH_ISSUE_URL);
    eprintln!("If your browser did not open automatically, go to {klyra_GH_ISSUE_URL}");

    Ok(())
}

pub async fn update_cargo_klyra(preview: bool) -> Result<()> {
    if preview {
        let _ = tokio::process::Command::new("cargo")
            .args(["install", "cargo-klyra", "--git", klyra_GH_REPO_URL])
            .kill_on_drop(true)
            .spawn()
            .context("Failed to spawn cargo install process")?
            .wait()
            .await
            .context("Failed to wait on cargo install process")?;

        return Ok(());
    }

    #[cfg(target_family = "unix")]
    let _ = tokio::process::Command::new("bash")
        .args(["-c", "curl -sSfL https://www.klyra.dev/install | bash"])
        .kill_on_drop(true)
        .spawn()
        .context("Failed to spawn bash update process")?
        .wait()
        .await
        .context("Failed to wait on bash update process")?;

    #[cfg(target_family = "windows")]
    let _ = tokio::process::Command::new("powershell")
        .args(["-Command", "iwr https://www.klyra.dev/install-win | iex"])
        .kill_on_drop(true)
        .spawn()
        .context("Failed to spawn powershell update process")?
        .wait()
        .await
        .context("Failed to wait on powershell update process")?;

    Ok(())
}

pub async fn read_ws_until_text<T>(rx: &mut T) -> Result<Option<String>>
where
    T: StreamExt<Item = tungstenite::Result<Message>> + Unpin,
{
    while let Some(Ok(msg)) = rx.next().await {
        if let Message::Text(s) = msg {
            return Ok(Some(s));
        }
    }

    Ok(None)
}
