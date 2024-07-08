mod models;
mod services;

use actix_web::middleware::Logger;
use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use env_logger::Env;
use services::database_config::Database;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello gbubemi")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize logger with environment variable support
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    // Initialize the database connection
    let db = Database::init().await.expect("error");
    // Clone the database connection to share it across handlers
    let db_data = Data::new(db);

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .app_data(db_data.clone())
            .service(hello)
    })
    .bind(("localhost", 5001))?
    .run()
    .await
}
