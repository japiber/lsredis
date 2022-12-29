use actix_files::NamedFile;
// Add HttpRequest and HttpResponse
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_actors::ws;

// Import the WebSocket logic we wrote earlier.
use crate::signaling::controller::SignalingWebSocket;

async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

// WebSocket handshake and start `MyWebSocket` actor.
async fn websocket(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(SignalingWebSocket::new(), &req, stream)
}

pub async fn start() -> std::io::Result<()> {
    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/").to(index))
            // Add the WebSocket route
            .service(web::resource("/ws").route(web::get().to(websocket)))
            .wrap(middleware::Logger::default())
    })
        .workers(2)
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
