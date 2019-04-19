extern crate tasks_github_bot;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use tasks_github_bot::app;

fn main() {
    pretty_env_logger::init();
    debug!("Logger initialized");
    app::run();
}
