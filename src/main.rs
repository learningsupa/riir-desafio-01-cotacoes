use axum::routing::get;
use axum::Router;
use hyper::Server;

async fn service_a() -> &'static str {
    "serviço a"
}

async fn service_b() -> &'static str {
    "serviço b"
}

async fn service_c() -> &'static str {
    "serviço b"
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new()
        .route("/servico-a/cotacao", get(service_a))
        .route("/servico-b/cotacao", get(service_b))
        .route("/servico-c/cotacao", get(service_c));

    Server::bind(&"0.0.0.0:3000".parse()?)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
