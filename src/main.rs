use actix_web::{get, web, App, HttpServer, Responder};

#[get("/hello")]
async fn hello_world() -> impl Responder {
    format!("Hello World!")
}

#[get("/health")]
async fn health_check() -> impl Responder {
    format!("active")
}

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[get("/doc/func")]
async fn func() -> impl Responder {
    format!("func")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello_world)
            .service(health_check)
            .service(greet)
            .service(func)
        })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
