use crossterm::style::Stylize;

/// Used to handle the case of getting the last deployment or getting
/// the logs failed.
pub fn get_logs_failure(err: anyhow::Error, title: &str) -> anyhow::Error {
    println!();
    println!("{}", title.red());
    println!();
    println!("Please check your project status and deployments:");
    println!();
    println!("1. cargo klyra project status");
    println!();
    println!("2. cargo klyra deployment list");
    println!();
    println!(
        "If getting the logs fails repeatedly, please try restarting your project before getting the logs again or contacting the team on the Discord server:"
    );
    println!();
    println!("cargo klyra project restart");
    err
}
