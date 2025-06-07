mod controller;
mod domain;
mod repository;
mod route;
mod service;

use std::sync::Arc;
use std::time::Duration;
use axum::Router;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use repository::task_repository::TaskRepositoryImpl;
use service::task_service::TaskService;
use route::route::create_routes;

#[tokio::main]
async fn main() {
    dotenv().expect("Cannot read .env");

    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_string());
    let database_url = std::env::var("DATABASE_URL").expect("Missing DATABASE_URL");

    let db_pool = PgPoolOptions::new()
        .max_connections(64)
        .acquire_timeout(Duration::from_secs(5))
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    let repo = Arc::new(TaskRepositoryImpl { db: db_pool });
    let service = Arc::new(TaskService { repo });

    let listener = TcpListener::bind(&server_address).await.unwrap();
    println!("Listening on {}", &server_address);

    let app: Router = create_routes(service);

    axum::serve(listener, app).await.unwrap();
}
