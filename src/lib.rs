extern crate reqwest;
extern crate pretty_env_logger;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate log;

pub mod app;
mod github;
mod bugzilla;
mod updater;