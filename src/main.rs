mod db;
mod handlers;
mod models;
mod mw;
mod orm;
mod routes;

use db::postgres::connect_db;
use dotenvy::dotenv;
use dotenvy_macro::dotenv;
use service_utils_rs::{services::jwt::Jwt, settings::Settings};

// use chrono::{Local, TimeZone, Utc};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let settings = Settings::new("config/services.toml").unwrap();
    let jwt = Jwt::new(settings.jwt);

    let database_uri = dotenv!("DATABASE_URL");
    let database: sea_orm::prelude::DatabaseConnection = connect_db(database_uri).await.unwrap();

    let routes = routes::create_routes(database, jwt);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:13000")
        .await
        .unwrap();

    println!("Server running on port 13000");
    axum::serve(listener, routes).await.unwrap();
}
