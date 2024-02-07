mod api;
mod mirror;
mod srv;
mod types;

use lazy_static::lazy_static;
use rocket::{routes, Config};
use sqlx::sqlite::SqlitePoolOptions;
use std::net::Ipv4Addr;

lazy_static! {
    pub static ref PUBLIC_URL_PREFIX: String = dotenvy::var("PUBLIC_URL_PREFIX").unwrap();
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let config = Config {
        port: 58532,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::default()
    };

    let db_url = dotenvy::var("DATABASE_URL")?;

    // this causes lazy_static to initialize the variable, which will panic if the env var is not set
    println!("public url prefix: {}", *PUBLIC_URL_PREFIX);

    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    // check if initial sync is finished
    let initial_sync_finished = mirror::check_initial_sync(pool.clone()).await?;

    if !initial_sync_finished {
        println!("performing initial sync");
        mirror::initial_sync(pool.clone()).await?;
    }

    let _rocket = rocket::build()
        .manage(pool)
        .mount("/", routes![srv::version, api::get_song])
        .configure(config)
        .launch()
        .await?;

    Ok(())
}
