use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};

#[get("/health")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/echo/{message}")]
async fn echo(message: web::Path<String>) -> impl Responder {
    let message_str = message.to_string();
    HttpResponse::Ok().body(format!("Echo: {}", message_str))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(health_check)
            .service(hello)
            .service(echo)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
