#[cfg(feature = "ssr")]
pub mod server;

#[cfg(feature = "ssr")]
pub mod routes;

#[cfg(feature = "ssr")]
pub mod state;

pub mod shared;
pub mod components;
pub mod app;
