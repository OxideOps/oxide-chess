#[cfg(feature = "ssr")]
mod auth;
#[cfg(feature = "ssr")]
mod game_socket;
#[cfg(feature = "ssr")]
mod launcher;
#[cfg(feature = "ssr")]
mod mailer;
pub(crate) mod server_functions;

#[cfg(feature = "ssr")]
pub use launcher::launch;
