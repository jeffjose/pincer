use poem::{get, handler, listener::TcpListener, web::Html, web::Json, web::Path, Route, Server};
use reqwest;

use serde::Serialize;

#[derive(Serialize)]
struct Response {
    name: String,
}

#[handler]
async fn proxy(Path(url): Path<String>) -> Html<String> {
    //format!("{}", name)

    let fullpath = format!("https://{url}");
    let result = reqwest::get(fullpath).await.unwrap().text().await.unwrap();

    Html(result)
}

#[handler]
async fn json(Path(name): Path<String>) -> Json<Response> {
    Json(Response { name })
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/proxy/*url", get(proxy))
        .at("/json/:name", get(json));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("pincer")
        .run(app)
        .await
}
