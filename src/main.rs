use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    sync::Client,
};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

async fn connect_db() -> mongodb::error::Result<()> {
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    let mut client_options = ClientOptions::parse(uri).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;
    client
        .database("admin")
        .run_command(doc! { "ping": 1 })
        .await?;

    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    match connect_db().await {
        Ok(_) => println!("Successfully connected to the database."),
        Err(e) => println!("Failed to connect to the database: {:?}", e),
    }

    println!("Starting web server...");

    let server = HttpServer::new(|| App::new().route("/", web::get().to(greet)))
        .bind(("0.0.0.0", 5001))
        .unwrap()
        .run();

    println!("Server running at http://localhost:5001/");

    server.await
}
