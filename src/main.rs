mod api;
mod mirror;
mod srv;
mod types;

use rocket::{routes, Config};
use sqlx::sqlite::SqlitePoolOptions;
use std::net::Ipv4Addr;

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let config = Config {
        port: 58532,
        address: Ipv4Addr::new(0, 0, 0, 0).into(),
        ..Config::default()
    };

    let db_url = dotenvy::var("DATABASE_URL")?;

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
