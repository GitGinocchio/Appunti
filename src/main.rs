mod collector;
mod preprocessor;
mod writer;
mod environment;

use crate::{collector::Collector, environment::Environment};

#[tokio::main]
async fn main() {
    let env = Environment::load().expect("Error loading environment");
    let collector = Collector::new(env).expect("Invalid Notion api key");

    collector.fetch().await.expect("Error fetching");

    println!("Hello, world!");
}
