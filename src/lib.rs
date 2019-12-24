//! Make cool Telegram bots with Rust easily. Here is a simple echo bot:
//!
//! ```no_run
//! use tbot::prelude::*;
//!
//! # /*
//! #[tokio::main]
//! async fn main() {
//! # */
//! # // is there a way to enable `tokio/macros` for examples?
//! # async fn bot() {
//!     let mut bot = tbot::from_env!("BOT_TOKEN").event_loop();
//!
//!     bot.text(|context| {
//!         async move {
//!             let echo = &context.text.value;
//!             let call_result = context.send_message(echo).call().await;
//!
//!             if let Err(err) = call_result {
//!                 dbg!(err);
//!             }
//!         }
//!     });
//!
//!     bot.polling().start().await.unwrap();
//! }
//! ```
//!
//! If you're new to `tbot`, we recommend you go through the [tutorial] first.
//! We also have several [How-to guides][how-to] with snippets to solve your
//! problems.
//!
//! If you have a question, ask it in [our group] on Telegram. If you find
//! a bug, fill an issue on either our [GitLab] or [GitHub] repository.
//!
//! [our group]: https://t.me/tbot_group
//! [tutorial]: https://gitlab.com/SnejUgal/tbot/wikis/Tutorial
//! [how-to]: https://gitlab.com/SnejUgal/tbot/wikis/How-to
//! [GitLab]: https://gitlab.com/SnejUgal/tbot
//! [GitHub]: https://github.com/SnejUgal/tbot

#![deny(
    future_incompatible,
    nonstandard_style,
    missing_docs,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(clippy::multiple_crate_versions)] // can't do much
#![allow(clippy::needless_doctest_main)] // that's where you're wrong, kiddo

#[cfg(all(feature = "tls", feature = "rustls"))]
compile_error!("`tls` and `rustls` features are mutually exclusive. You should enable only one of them");

mod bot;
mod download_file;
mod internal;
mod multipart;
mod token;

pub mod connectors;
pub mod contexts;
pub mod errors;
pub mod event_loop;
pub mod methods;
pub mod types;

use {download_file::download_file, multipart::Multipart, token::Token};

pub use {bot::Bot, event_loop::EventLoop};

pub mod prelude {
    //! Traits needed when working with `tbot`.
    pub use super::contexts::traits::*;
}
