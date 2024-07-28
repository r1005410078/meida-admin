use env_logger::Env;
use presentation::web::run;

mod common;
mod domain;
mod infrastructure;
mod presentation;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    run().await
}
