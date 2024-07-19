use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn greet() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting web server...");

    let server = HttpServer::new(|| App::new().route("/", web::get().to(greet)))
        .bind("0.0.0.0:5001")?
        .run();

    println!("Server running at http://localhost:5001/");

    server.await
}
