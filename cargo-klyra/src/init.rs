use std::{
    fs::{read_to_string, OpenOptions},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Context, Result};
use cargo_generate::{GenerateArgs, TemplatePath, Vcs};
use indoc::indoc;
use klyra_common::project::ProjectName;
use toml_edit::{value, Document};

#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::Display, strum::EnumIter)]
#[strum(serialize_all = "kebab-case")]
pub enum Template {
    ActixWeb,
    Axum,
    Poise,
    Poem,
    Rocket,
    Salvo,
    Serenity,
    Tide,
    Thruster,
    Tower,
    Warp,
    None,
}

impl Template {
    /// Returns a framework-specific struct that implements the trait `KlyraInit`
    /// for writing framework-specific dependencies to `Cargo.toml` and generating
    /// boilerplate code in `src/main.rs`.
    pub fn init_config(&self) -> Box<dyn KlyraInit> {
        use Template::*;
        match self {
            ActixWeb => Box::new(KlyraInitActixWeb),
            Axum => Box::new(KlyraInitAxum),
            Rocket => Box::new(KlyraInitRocket),
            Tide => Box::new(KlyraInitTide),
            Tower => Box::new(KlyraInitTower),
            Poem => Box::new(KlyraInitPoem),
            Salvo => Box::new(KlyraInitSalvo),
            Serenity => Box::new(KlyraInitSerenity),
            Poise => Box::new(KlyraInitPoise),
            Warp => Box::new(KlyraInitWarp),
            Thruster => Box::new(KlyraInitThruster),
            None => Box::new(KlyraInitNoOp),
        }
    }
}

pub trait KlyraInit {
    fn get_repo_url(&self) -> &'static str;
    fn get_sub_path(&self) -> Option<&'static str>;
}

pub struct KlyraInitActixWeb;

impl KlyraInit for KlyraInitActixWeb {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/klyra-hq/klyra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("actix-web/hello-world")
    }
}

pub struct KlyraInitAxum;

impl KlyraInit for KlyraInitAxum {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/klyra-hq/klyra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("axum/hello-world")
    }
}

pub struct KlyraInitRocket;

impl KlyraInit for KlyraInitRocket {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/klyra-hq/klyra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("rocket/hello-world")
    }
}

pub struct KlyraInitTide;

impl KlyraInit for KlyraInitTide {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/klyra-hq/klyra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("tide/hello-world")
    }
}

pub struct KlyraInitPoem;

impl KlyraInit for KlyraInitPoem {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/klyra-hq/klyra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("poem/hello-world")
    }
}

pub struct KlyraInitSalvo;

impl KlyraInit for KlyraInitSalvo {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/klyra-hq/klyra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("salvo/hello-world")
    }
}

pub struct KlyraInitSerenity;

impl KlyraInit for KlyraInitSerenity {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/klyra-hq/klyra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("serenity/hello-world")
    }
}

pub struct KlyraInitPoise;

impl KlyraInit for KlyraInitPoise {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/klyra-hq/klyra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("poise/hello-world")
    }
}

pub struct KlyraInitTower;

impl KlyraInit for KlyraInitTower {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/klyra-hq/klyra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("tower/hello-world")
    }
}

pub struct KlyraInitWarp;

impl KlyraInit for KlyraInitWarp {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/klyra-hq/klyra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("warp/hello-world")
    }
}

pub struct KlyraInitThruster;

impl KlyraInit for KlyraInitThruster {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/klyra-hq/klyra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("thruster/hello-world")
    }
}

pub struct KlyraInitNoOp;
impl KlyraInit for KlyraInitNoOp {
    fn get_repo_url(&self) -> &'static str {
        "http://github.com/klyra-hq/klyra-examples.git"
    }

    fn get_sub_path(&self) -> Option<&'static str> {
        Some("custom/none")
    }
}

pub fn cargo_generate(path: PathBuf, name: &ProjectName, framework: Template) -> Result<()> {
    let config = framework.init_config();

    println!(r#"    Creating project "{name}" in {path:?}"#);
    let generate_args = GenerateArgs {
        init: true,
        template_path: TemplatePath {
            git: Some(config.get_repo_url().to_string()),
            auto_path: config.get_sub_path().map(str::to_string),
            ..Default::default()
        },
        name: Some(name.to_string()), // appears to do nothing...
        destination: Some(path.clone()),
        vcs: Some(Vcs::Git),
        ..Default::default()
    };
    cargo_generate::generate(generate_args)
        .with_context(|| "Failed to initialize with cargo generate.")?;

    set_crate_name(&path, name.as_str()).with_context(|| "Failed to set crate name.")?;
    remove_klyra_toml(&path);
    create_gitignore_file(&path).with_context(|| "Failed to create .gitignore file.")?;

    Ok(())
}

// since I can't get cargo-generate to do this for me...
fn set_crate_name(path: &Path, name: &str) -> Result<()> {
    // read the Cargo.toml file
    let mut path = path.to_path_buf();
    path.push("Cargo.toml");

    let toml_str = read_to_string(&path)?;
    let mut doc = toml_str.parse::<Document>()?;

    // change the name
    doc["package"]["name"] = value(name);

    // write the Cargo.toml file back out
    std::fs::write(&path, doc.to_string())?;

    Ok(())
}

/*
Currently Klyra.toml only has a project name override.
This project name will already be in use, so the file is useless.

If we start putting more things in Klyra.toml we may wish to re-evaluate.
*/
fn remove_klyra_toml(path: &Path) {
    let mut path = path.to_path_buf();
    path.push("Klyra.toml");

    // this file only exists for some of the examples, it's fine if we don't find it
    _ = std::fs::remove_file(path);
}

fn create_gitignore_file(path: &Path) -> Result<()> {
    let mut path = path.to_path_buf();
    path.push(".gitignore");

    let mut file = match OpenOptions::new().create_new(true).write(true).open(path) {
        Ok(f) => f,
        Err(e) => {
            match e.kind() {
                ErrorKind::AlreadyExists => {
                    // if the example already has a .gitignore file, just use that
                    return Ok(());
                }
                _ => {
                    return Err(anyhow!(e));
                }
            }
        }
    };

    file.write_all(indoc! {b"
        /target
        Secrets*.toml
    "})?;

    Ok(())
}
