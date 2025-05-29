use crossterm::style::Stylize;

/// Used when the deploy request doesn't succeed.
pub fn deploy_request_failure(err: anyhow::Error) -> anyhow::Error {
    println!();
    println!("{}", "Deploy request failed".red());
    println!();
    println!("Please check your project status and deployments:");
    println!();
    println!("1. cargo klyra project status");
    println!();
    println!("2. cargo klyra deployment list");
    println!();
    println!(
        "If deploying fails repeatedly, please try restarting your project before deploying again or contacting the team on the Discord server:"
    );
    println!();
    println!("cargo klyra project restart");
    err
}

/// Especially used for cases where the deployment fails after the
/// deploy request went through (e.g. following the deployment logs, checking
/// the deployment state).
pub fn deployment_setup_failure(err: anyhow::Error, title: &str) -> anyhow::Error {
    println!();
    println!("{}", title.dark_red());
    println!();
    println!(
        "Please check your project status and if the last deployment is recent and is running:"
    );
    println!();
    println!("1. cargo klyra project status");
    println!();
    println!("2. cargo klyra deployment list");
    println!();
    println!("You should be able to get the logs of the deployment by running:");
    println!();
    println!("cargo klyra logs");
    println!();
    println!("Or follow the logs of the deployment by running:");
    println!();
    println!("cargo klyra logs --follow");
    println!("If the last deployment is not recent or is not running, please try deploying again  or contacting the team on the Discord server:");
    println!();
    println!("cargo klyra deploy");
    println!();
    println!("Or restart the project before deploying again:");
    println!();
    println!("cargo klyra project restart");
    err
}
