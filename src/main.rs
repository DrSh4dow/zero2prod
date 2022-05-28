use actix_web::{web, App, HttpRequest, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!\n", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // HttpServer it is the struct responsible for all
    // the transport level concerns.
    HttpServer::new(|| {
        // App it is where all the application logic lives
        // This includes routing, middleware, request handlers, etc
        // The app struct uses a fluent API.
        App::new()
            .route("/", web::get().to(greet))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
