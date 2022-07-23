use axum::extract::Query;
use axum::routing::{get, post};
use axum::{Json, Router};
use hyper::{Server, StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use uuid::Uuid;

#[derive(Deserialize)]
struct QueryServiceA {
    curr: String,
}

async fn service_a(Query(query): Query<QueryServiceA>) -> (StatusCode, Json<Value>) {
    if query.curr == "" {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "message": "Oh, no! Você precisa informar o parâmetro 'curr'",
                "success": false,
            })),
        )
    } else {
        (
            StatusCode::OK,
            Json(json!({
                "fator": 1000,
                "currency": query.curr,
                "valor": 0
            })),
        )
    }
}

#[derive(Deserialize)]
struct QueryServiceB {
    moeda: String,
}

async fn service_b(Query(query): Query<QueryServiceB>) -> (StatusCode, Json<Value>) {
    if query.moeda == "" {
        (
            StatusCode::BAD_REQUEST,
            Json(json!({"erro": "Oh, no! Você precisa informar o parâmetro 'moeda'" })),
        )
    } else {
        (
            StatusCode::OK,
            Json(json!({
                "symbol": "💵",
                "moeda": query.moeda,
                "cotacao": 0
            })),
        )
    }
}

#[derive(Deserialize)]
struct BodyServiceC {
    callback: url::Url,
    tipo: String,
}

async fn service_c(Json(payload): Json<BodyServiceC>) -> (StatusCode, Json<Value>) {
    if payload.tipo == "" {
        (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(json!({
                "mood": "⛔",
                "erro": "Oh, no! Você precisa informar o parâmetro 'tipo'!",
                "dica": "Provavelmente, você quer usar http://172.17.0.1:<porta> ou http://host.docker.internal:<porta> para que o docker acesse seu ambiente :)",
            })),
        )
    } else {
        (
            StatusCode::OK,
            Json(json!({
                "mood": "✅",
                "cid": Uuid::new_v4(),
                "mensagem": format!("quando a cotação finalizar, uma requisição para {} será feita", payload.callback),
            })),
        )
    }
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
