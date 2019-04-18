extern crate tasks_github_bot;
extern crate pretty_env_logger;
#[macro_use]
extern crate log;

use std::env;

use tasks_github_bot::app;

fn main() {
    pretty_env_logger::init();
    debug!("Logger initialized");

    let token = match env::var("GITHUB_TOKEN") {
      Ok(val) => val,
      Err(_e) => {
        error!("No GITHUB_TOKEN set");
        String::new()
      },
    };

    app::run(token);
}
