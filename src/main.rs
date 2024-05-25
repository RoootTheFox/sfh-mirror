mod api;
mod mirror;
mod srv;
mod types;

use lazy_static::lazy_static;
use rocket::fairing::AdHoc;
use rocket::{routes, Build, Config, Rocket};
use rocket_db_pools::Database;
use std::net::Ipv4Addr;

lazy_static! {
    pub static ref PUBLIC_URL_PREFIX: String = dotenvy::var("PUBLIC_URL_PREFIX").unwrap();
}

#[derive(Database)]
#[database("sqlx")]
struct Db(sqlx::MySqlPool);

async fn run_migrations(rocket: Rocket<Build>) -> rocket::fairing::Result {
    if let Some(db) = Db::fetch(&rocket) {
        match sqlx::migrate!().run(&db.0).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("failed to run migrations: {:?}", e);
                return Err(rocket);
            }
        }

        let initial_sync_finished = match mirror::check_initial_sync(&db.0).await {
            Ok(finished) => finished,
            Err(e) => {
                eprintln!("failed to check initial sync status: {:?}", e);
                return Err(rocket);
            }
        };

        if !initial_sync_finished {
            println!("performing initial sync");
            match mirror::initial_sync(&db.0).await {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("failed to perform initial sync: {:?}", e);
                    return Err(rocket);
                }
            }
        }

        Ok(rocket)
    } else {
        Err(rocket)
    }
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let db_url = dotenvy::var("DATABASE_URL")?;
    dbg!(&db_url);
    let figment = Config::figment()
        .merge(("port", 58532))
        .merge(("address", Ipv4Addr::from([0, 0, 0, 0])))
        .merge((
            "databases.sqlx",
            rocket_db_pools::Config {
                url: db_url,
                min_connections: None,
                max_connections: 150,
                connect_timeout: 3,
                idle_timeout: None,
            },
        ));

    // this causes lazy_static to initialize the variable, which will panic if the env var is not set
    println!("public url prefix: {}", *PUBLIC_URL_PREFIX);

    let _rocket = rocket::build()
        .attach(Db::init())
        .attach(AdHoc::try_on_ignite("DB Migrations", run_migrations))
        .mount(
            "/",
            routes![
                srv::version,
                api::get_song,
                api::get_songs_for_level,
                api::get_songs_with_id
            ],
        )
        .configure(figment)
        .launch()
        .await?;

    Ok(())
}
