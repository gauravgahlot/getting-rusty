use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log::info;

const HOST_ADDR: &str = "0.0.0.0";
const HOST_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // initialize the env_logger
    env_logger::init();

    info!("starting server at {}:{}", HOST_ADDR, HOST_PORT);

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello_world))
    })
    .bind((HOST_ADDR, HOST_PORT))?
    .run()
    .await
}

async fn hello_world(r: HttpRequest) -> impl Responder {
    info!("{:?}", r);
    HttpResponse::Ok().body("Hello, World!!")
}

