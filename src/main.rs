use axum::extract::Query;
use axum::routing::{get, post};
use axum::{Json, Router};
use hyper::Server;
use serde::Deserialize;

#[derive(Deserialize)]
struct QueryServiceA {
    curr: String
}

async fn service_a(Query(_query): Query<QueryServiceA>) -> &'static str {
    "serviço a"
}

#[derive(Deserialize)]
struct QueryServiceB {
    moeda: String
}

async fn service_b(Query(_query): Query<QueryServiceB>) -> &'static str {
    "serviço b"
}

#[derive(Deserialize)]
struct BodyServiceC {
    callback: String,
    tipo: String
}

async fn service_c(Json(_payload): Json<BodyServiceC>) -> &'static str {
    "serviço c"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/servico-a/cotacao", get(service_a))
        .route("/servico-b/cotacao", get(service_b))
        .route("/servico-c/cotacao", post(service_c));

    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
