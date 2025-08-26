use crate::{
    __internals::{Loader, Runner},
    rt,
};

/// Uses simple arg parsing logic instead of clap to reduce dependency weight.
/// The rest of the args are parsed in `RuntimeEnvVars`.
fn initial_args_and_env_check() -> anyhow::Result<()> {
    if std::env::args().any(|arg| arg == "--port") {
        anyhow::bail!("Outdated argument detected (--port). Upgrade your Klyra CLI.");
    }

    if std::env::var("klyra_ENV").is_err() {
        anyhow::bail!("klyra_ENV is required to be set on klyra.dev");
    }

    Ok(())
}

pub async fn start(
    loader: impl Loader + Send + 'static,
    runner: impl Runner + Send + 'static,
    crate_name: &'static str,
    package_version: &'static str,
) {
    // `--version` overrides any other arguments. Used by cargo-klyra to check compatibility on local runs.
    if std::env::args().any(|arg| arg == "--version") {
        println!("{}", crate::VERSION_STRING);
        return;
    }

    println!(
        "{} starting: {} {}",
        crate::VERSION_STRING,
        crate_name,
        package_version
    );

    if let Err(e) = initial_args_and_env_check() {
        eprintln!("ERROR: Runtime failed to parse args: {e}");
        let help_str = "[HINT]: Run your Klyra app with `klyra run`";
        let wrapper_str = "-".repeat(help_str.len());
        eprintln!("{wrapper_str}\n{help_str}\n{wrapper_str}");
        return;
    }

    // this is handled after arg parsing to not interfere with --version above
    #[cfg(all(feature = "setup-tracing", not(feature = "setup-otel-exporter")))]
    {
        use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};
        registry()
            .with(fmt::layer().without_time())
            .with(
                // let user override RUST_LOG in local run if they want to
                EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    // otherwise use our default
                    format!("info,{}=debug", crate_name).into()
                }),
            )
            .init();
    }

    #[cfg(feature = "setup-otel-exporter")]
    let _guard = {
        use tracing_subscriber::{fmt, prelude::*, registry, EnvFilter};
        let (layers, guard) =
            crate::telemetry::otel_tracing_subscriber(crate_name, package_version);

        registry()
            .with(layers)
            .with(fmt::layer().without_time())
            .with(
                // let user override RUST_LOG in local run if they want to
                EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                    // otherwise use our default
                    format!("info,{}=debug", crate_name).into()
                }),
            )
            .init();

        guard
    };

    #[cfg(any(feature = "setup-tracing", feature = "setup-otel-exporter"))]
    tracing::warn!("Default tracing subscriber initialized (https://docs.klyra.dev/docs/logs)");

    rt::start(loader, runner).await
}
