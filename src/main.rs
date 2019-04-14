extern crate tasks_github_bot;

use std::env;

use tasks_github_bot::app;

fn main() {
    let token = match env::var("GITHUB_TOKEN") {
      Ok(val) => val,
      Err(_e) => String::new(),
    };

    app::run(token);
}
