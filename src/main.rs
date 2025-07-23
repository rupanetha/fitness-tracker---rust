mod types;
mod calculator;
mod tracker;
mod handlers;
mod display;


use types::{TodayStats, Workout};
use tracker::FitnessTracker;
use actix_web::{App, HttpServer, web};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use std::env;
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use actix_identity::IdentityMiddleware;
use actix_web::{cookie::Key};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let secret_key = Key::generate(); // For real apps, store in env

    println!("🚀 Server running on http://localhost:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret_key.clone(),
            ))
            .wrap(IdentityMiddleware::default())
            .service(handlers::register)
            .service(handlers::login)
            .service(handlers::logout)
            .service(handlers::summary)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}