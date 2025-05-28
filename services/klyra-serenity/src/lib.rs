//! Klyra service integration for the Serenity discord bot framework.
//!
//! ## Example
//!
//! ```rust,no_run
//! use anyhow::anyhow;
//! use serenity::async_trait;
//! use serenity::model::channel::Message;
//! use serenity::model::gateway::Ready;
//! use serenity::prelude::*;
//! use klyra_secrets::SecretStore;
//! use tracing::{error, info};
//!
//! struct Bot;
//!
//! #[async_trait]
//! impl EventHandler for Bot {
//!     async fn message(&self, ctx: Context, msg: Message) {
//!         if msg.content == "!hello" {
//!             if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
//!                 error!("Error sending message: {:?}", e);
//!             }
//!         }
//!     }
//!
//!     async fn ready(&self, _: Context, ready: Ready) {
//!         info!("{} is connected!", ready.user.name);
//!     }
//! }
//!
//! #[klyra_runtime::main]
//! async fn serenity(
//!     #[klyra_secrets::Secrets] secret_store: SecretStore,
//! ) -> klyra_serenity::KlyraSerenity {
//!     // Get the discord token set in `Secrets.toml`
//!     let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
//!         token
//!     } else {
//!         return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
//!     };
//!
//!     // Set gateway intents, which decides what events the bot will be notified about
//!     let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
//!
//!     let client = Client::builder(&token, intents)
//!         .event_handler(Bot)
//!         .await
//!         .expect("Err creating client");
//!
//!     Ok(client.into())
//! }
//! ```

use klyra_runtime::{CustomError, Error};
use std::net::SocketAddr;

/// A wrapper type for [serenity::Client] so we can implement [klyra_runtime::Service] for it.
pub struct SerenityService(pub serenity::Client);

#[klyra_runtime::async_trait]
impl klyra_runtime::Service for SerenityService {
    /// Takes the client that is returned by the user in their [klyra_runtime::main] function
    /// and starts it.
    async fn bind(mut self, _addr: SocketAddr) -> Result<(), Error> {
        self.0.start_autosharded().await.map_err(CustomError::new)?;

        Ok(())
    }
}

impl From<serenity::Client> for SerenityService {
    fn from(router: serenity::Client) -> Self {
        Self(router)
    }
}

/// Return type from the `[klyra_runtime::main]` macro for a Serenity-based service.
///
/// ## Example
///
/// ```rust,no_run
/// # use anyhow::anyhow;
/// # use serenity::async_trait;
/// # use serenity::model::channel::Message;
/// # use serenity::model::gateway::Ready;
/// # use serenity::prelude::*;
/// # use klyra_secrets::SecretStore;
/// # use tracing::{error, info};
/// # struct Bot;
/// # #[async_trait]
/// # impl EventHandler for Bot {
/// #     async fn message(&self, ctx: Context, msg: Message) {
/// #         if msg.content == "!hello" {
/// #             if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
/// #                 error!("Error sending message: {:?}", e);
/// #             }
/// #         }
/// #     }
/// #     async fn ready(&self, _: Context, ready: Ready) {
/// #         info!("{} is connected!", ready.user.name);
/// #     }
/// # }
///
/// #[klyra_runtime::main]
/// async fn serenity(
///     #[klyra_secrets::Secrets] secret_store: SecretStore,
/// ) -> klyra_serenity::KlyraSerenity {
///     // Get the discord token set in `Secrets.toml`
///     let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
///         token
///     } else {
///         return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
///     };
///
///     // Set gateway intents, which decides what events the bot will be notified about
///     let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
///
///     let client = Client::builder(&token, intents)
///         .event_handler(Bot)
///         .await
///         .expect("Err creating client");
///
///     Ok(client.into())
/// }
/// ```
pub type KlyraSerenity = Result<SerenityService, Error>;
