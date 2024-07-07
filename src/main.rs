mod models;
mod services;

use actix_web::{get, web::Data, App, HttpResponse, HttpServer, Responder};
use services::database_config::Database;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello gbubemi")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the database connection
    let db = Database::init().await.expect("error");
    // Clone the database connection to share it across handlers
    let db_data = Data::new(db);

    // Start the HTTP server
    HttpServer::new(move || App::new().app_data(db_data.clone()).service(hello))
        .bind(("localhost", 5001))?
        .run()
        .await
}
