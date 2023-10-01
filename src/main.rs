use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::post;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/pessoas", post(create_person))
        .route("/pessoas/:id", get(find_person))
        .route("/pessoas", get(search_people))
        .route("/contagem-pessoa", get(count_people));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn create_person() -> impl IntoResponse {
    (StatusCode::OK, "Cria pessoa")
}

async fn find_person() -> impl IntoResponse {
    (StatusCode::OK, "Busca pessoa")
}

async fn search_people() -> impl IntoResponse {
    (StatusCode::OK, "Busca pessoas")
}

async fn count_people() -> impl IntoResponse {
    (StatusCode::OK, "Contagem de pessoas")
}
