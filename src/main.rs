mod api;
mod mirror;
mod srv;
mod types;

use lazy_static::lazy_static;
use rocket::fairing::AdHoc;
use rocket::{routes, tokio, Build, Config, Rocket};
use rocket_db_pools::Database;
use std::net::Ipv4Addr;
use std::time::Duration;

lazy_static! {
    pub static ref PUBLIC_URL_PREFIX: String = dotenvy::var("PUBLIC_URL_PREFIX").unwrap();
}

#[derive(Database)]
#[database("sqlx")]
struct Db(sqlx::SqlitePool);

async fn run_migrations(rocket: Rocket<Build>) -> rocket::fairing::Result {
    if let Some(db) = Db::fetch(&rocket) {
        match sqlx::migrate!().run(&db.0).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("failed to run migrations: {:?}", e);
                return Err(rocket);
            }
        }

        println!("performing sync");
        match mirror::sync(&db.0).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("failed to perform sync: {:?}", e);
                return Err(rocket);
            }
        }

        // start syncing periodically :3 :3 :3 :3 :3 help the :3 is taking over :3 :3 :3c
        // >:3c
        // waaaa

        let db_ref = db.0.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(240));

            loop {
                interval.tick().await;
                println!("periodic sync starting wee");
                match mirror::sync(&db_ref).await {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("failed to perform sync: {:?}", e);
                    }
                }
            }
        });

        Ok(rocket)
    } else {
        Err(rocket)
    }
}

#[rocket::main]
async fn main() -> anyhow::Result<()> {
    let db_url = dotenvy::var("DATABASE_URL")?;

    let figment = Config::figment()
        .merge(("port", 58532))
        .merge(("address", Ipv4Addr::from([0, 0, 0, 0])))
        .merge((
            "databases.sqlx",
            rocket_db_pools::Config {
                url: db_url,
                min_connections: None,
                max_connections: 1024,
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
