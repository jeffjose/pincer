use poem::{
    error::BadRequest, get, handler, listener::TcpListener, web::Json, web::Path, Result, Route,
    Server,
};
use reqwest::{self, Method};

use serde::Serialize;

#[derive(Serialize)]
struct Response {
    name: String,
}

#[handler]
async fn proxy(req: &poem::Request, _body: poem::Body) -> Result<poem::Response> {
    //let fullpath = format!("https://{url}");
    //let result = reqwest::get(fullpath).await.unwrap().text().await.unwrap();

    //let path = req.path_params::<String>()?;

    let path = req
        .uri()
        .path()
        .replace("/proxy", "")
        .replace("https://", "")
        .replace("http://", "");
    let query = match req.uri().query() {
        Some(q) => format!("?{}", q),
        None => String::from(""),
    };

    let url = format!("https://{path}{query}",);

    println!("[REQ]: {url}");

    let cli = reqwest::Client::new();
    let resp = cli
        .request(Method::GET, url)
        .send()
        .await
        .map_err(BadRequest)?;

    let mut r = poem::Response::default();
    r.set_status(resp.status());
    *r.headers_mut() = resp.headers().clone();
    r.set_body(resp.bytes().await.map_err(BadRequest)?);

    Ok(r)
}

#[handler]
async fn json(Path(name): Path<String>) -> Json<Response> {
    Json(Response { name })
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new()
        .at("/proxy/*url", proxy)
        .at("/json/:name", get(json));
    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .name("pincer")
        .run(app)
        .await
}
