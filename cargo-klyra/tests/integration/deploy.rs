use std::path::Path;

use cargo_klyra::{Args, Command, CommandOutcome, DeployArgs, ProjectArgs, Klyra};
use reqwest::StatusCode;
use test_context::test_context;
use tokiotest_httpserver::{handler::HandlerBuilder, HttpTestContext};

/// creates a `cargo-klyra` deploy instance with some reasonable defaults set.
async fn cargo_klyra_deploy(path: &str, api_url: String) -> anyhow::Result<CommandOutcome> {
    let working_directory = Path::new(path).to_path_buf();

    Klyra::new()
        .run(Args {
            api_url: Some(api_url),
            project_args: ProjectArgs {
                working_directory,
                name: None,
            },
            cmd: Command::Deploy(DeployArgs {
                allow_dirty: false,
                no_test: false,
            }),
        })
        .await
}

#[should_panic(
    expected = "Your klyra-service version is outdated. Update your klyra-service version to 1.2.5 and try to deploy again"
)]
#[test_context(HttpTestContext)]
#[tokio::test]
async fn deploy_when_version_is_outdated(ctx: &mut HttpTestContext) {
    ctx.add(
        HandlerBuilder::new("/test/version")
            .status_code(StatusCode::OK)
            .response("1.2.5".into())
            .build(),
    );
    let api_url = ctx.uri("/test").to_string();

    cargo_klyra_deploy("../examples/rocket/hello-world", api_url)
        .await
        .unwrap();
}

#[should_panic(expected = "not an absolute path")]
#[test_context(HttpTestContext)]
#[tokio::test]
async fn deploy_when_version_is_valid(ctx: &mut HttpTestContext) {
    ctx.add(
        HandlerBuilder::new("/test/version")
            .status_code(StatusCode::OK)
            .response(klyra_service::VERSION.into())
            .build(),
    );
    let api_url = ctx.uri("/test").to_string();

    cargo_klyra_deploy("../examples/rocket/hello-world", api_url)
        .await
        .unwrap();
}
