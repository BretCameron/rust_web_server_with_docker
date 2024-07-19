use std::env;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongodb_uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");

    let mut client_options = ClientOptions::parse(&mongodb_uri).await.unwrap();
    client_options.app_name = Some("ActixWebServer".to_string());

    let client = Client::with_options(client_options).unwrap();

    let database = client.database("test");
    let collections = database.list_collection_names().await.unwrap();

    for collection in collections {
        println!("{}", collection);
    }

    println!("Starting web server...");

    let server = HttpServer::new(|| App::new().route("/", web::get().to(greet)))
        .bind("0.0.0.0:5001")?
        .run();

    println!("Server running at http://localhost:5001/");

    server.await
}
