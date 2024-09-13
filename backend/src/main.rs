mod structs;
mod questions;
use serde_json::to_string;
use serde_json::Serializer;
use serde_json::Deserializer;
use questions::{easy, meddium, hard, very_hard};
use axum::{
    extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/:difficulty", get(get_questions));

    // run our app with hyper, listening globally on port 8080 
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_questions(Path(difficulty): Path<String>) -> Result<impl IntoResponse, StatusCode> {
   match difficulty.as_str() {
        "easy" => {
            let easy_string = to_string(&easy()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, easy_string))
        },
        "meddium" => {
            let meddium_string = to_string(&meddium()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, meddium_string))
        },
        "hard" => {
            let hard_string = to_string(&hard()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, hard_string))
        },
        "very hard" => {
            let very_hard_string = to_string(&very_hard()).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, very_hard_string))
        },

        _ => Err(StatusCode::NOT_IMPLEMENTED)
    } 
}
