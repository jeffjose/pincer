use poem::{get, handler, listener::TcpListener, web::Json, web::Path, Route, Server};

use serde::Serialize;

#[derive(Serialize)]
struct Response {
    name: String,
}

#[handler]
async fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[handler]
async fn json(Path(name): Path<String>) -> Json<Response> {
    Json(Response { name })
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/reddit/:name", get(hello))
        .at("/json/:name", get(json));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("pincer")
        .run(app)
        .await
}
