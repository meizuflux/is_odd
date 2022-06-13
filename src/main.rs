use axum::{routing::get, Router, extract, response::Json};
use std::net::SocketAddr;
use serde::Serialize;


#[derive(Serialize)]
struct IsOdd {
    is_odd: bool
}


#[tokio::main]
async fn main() {
    // build our application with a route
    let app = Router::new()
        .route("/", get(index))
        .route("/:number", get(is_odd));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> &'static str {
    "Routes: \n    /:number -> check if number is odd"
}

async fn is_odd(extract::Path(number): extract::Path<isize>) -> Json<IsOdd> {
    Json(IsOdd { is_odd: (number % 2) != 0 })
}