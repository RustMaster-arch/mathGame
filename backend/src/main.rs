use axum::{
    extract::{Path, State}, http::StatusCode, response::IntoResponse, routing::get, Json, Router
};
use serde_json::to_string;
use std::sync::Arc;
mod structs;
use structs::{ClientRequest, Question, UIQuestion};

// Define a struct to hold all the question sets
struct AppState {
    easy: Vec<Question>,
    medium: Vec<Question>,
    hard: Vec<Question>,
    very_hard: Vec<Question>,
}

#[tokio::main]
async fn main() {
    let easy: Vec<Question> = vec![
        Question::new("What is 5 + 3?", vec!["6", "7", "8", "9"], 2),
        Question::new("What is 10 - 4?", vec!["5", "6", "7", "8"], 1),
        Question::new("What is 6 * 2?", vec!["10", "11", "12", "13"], 2),
        Question::new("What is 15 / 3?", vec!["3", "4", "5", "6"], 2),
        Question::new("What is 9 + 6?", vec!["13", "14", "15", "16"], 2),
    ];

    let medium: Vec<Question> = vec![
        Question::new("What is the value of 7 * 8?", vec!["48", "54", "56", "63"], 2),
        Question::new("What is the square root of 144?", vec!["10", "12", "14", "16"], 1),
        Question::new("What is 25% of 200?", vec!["25", "40", "50", "75"], 2),
        Question::new("What is the value of 9^2?", vec!["64", "72", "81", "99"], 2),
        Question::new("What is the value of 5^3?", vec!["100", "120", "125", "150"], 2),
    ];

    let hard: Vec<Question> = vec![
        Question::new("What is the derivative of x^3 + 5x^2 - 2x + 1?", vec!["3x^2 + 10x - 2", "x^2 + 2x - 2", "4x^2 + 10x - 2", "6x^2 + 10x + 2"], 0),
        Question::new("What is the value of the integral of e^x from 0 to 1?", vec!["e - 1", "1", "e", "0"], 0),
        Question::new("Solve the equation: 2^(x+1) = 16", vec!["x = 3", "x = 2", "x = 4", "x = 5"], 0),
        Question::new("What is the limit of (x^2 - 1)/(x - 1) as x approaches 1?", vec!["2", "0", "3", "1"], 3),
        Question::new("If f(x) = 2x^2 + 3x, what is f'(x)?", vec!["4x + 3", "2x + 3", "4x + 6", "x + 3"], 0),
    ];

    let very_hard: Vec<Question> = vec![
        Question::new("What is the integral of e^(2x)?", vec!["e^(2x) + C", "e^(2x)/2 + C", "2e^(2x) + C", "e^(2x)/4 + C"], 1),
        Question::new("What is the limit of (x^2 - 1)/(x - 1) as x approaches 1?", vec!["0", "1", "2", "Undefined"], 2),
        Question::new("What is the solution to the differential equation y' + y = 0?", vec!["y = e^x", "y = e^(-x)", "y = x^2", "y = -x"], 1),
        Question::new("What is the determinant of the matrix [[1, 2], [3, 4]]?", vec!["-2", "2", "4", "-4"], 0),
        Question::new("What is the derivative of ln(x)?", vec!["1/x", "x", "ln(x)/x", "x^(-1)"], 0),
    ];

    let app_state = Arc::new(AppState {
        easy,
        medium,
        hard,
        very_hard,
    });

    let app = Router::new()
        .route("/:difficulty", get(questions))
        .route("/correct", get(correct))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn questions(Path(difficulty): Path<String>, State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    match difficulty.as_str() {
        "easy" => {
            let ui_question: Vec<UIQuestion> = state.easy.iter().map(|question| UIQuestion::from(question.clone())).collect();
            let state_string = to_string(&ui_question).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, [("content-type", "json")], state_string))
        },
        "medium" => {
            let ui_question: Vec<UIQuestion> = state.medium.iter().map(|question| UIQuestion::from(question.clone())).collect();
            let state_string = to_string(&ui_question).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, [("content-type", "json")], state_string))
        },
        "hard" => {
            let ui_question: Vec<UIQuestion> = state.hard.iter().map(|question| UIQuestion::from(question.clone())).collect();
            let state_string = to_string(&ui_question).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, [("content-type", "json")], state_string))
        },
        "very_hard" => {
            let ui_question: Vec<UIQuestion> = state.very_hard.iter().map(|question| UIQuestion::from(question.clone())).collect();
            let state_string = to_string(&ui_question).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            Ok((StatusCode::OK, [("content-type", "json")], state_string))
        },
        _ => Err(StatusCode::NOT_ACCEPTABLE),
    }
}

async fn correct(Json(ui_question): Json<ClientRequest>, State(state): State<Arc<AppState>>) -> Result<impl IntoResponse, StatusCode> {
    match ui_question.difficulty.as_str() {
        "easy" => {
            Ok(state.easy.iter().find_map(|question| {
                let index = question.correct_index;
                let mut res = to_string(&false).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

                if UIQuestion::from(question.clone()) == ui_question.ui_question {
                    if ui_question.answer == question.answers[index] {
                        res = "true".to_string();
                        return Some((StatusCode::OK, res))
                    }
                }
                return None
            }).unwrap())
        },

        "midium" => {
            Ok(state.medium.iter().find_map(|question| {
                let index = question.correct_index;
                let mut res = to_string(&false).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

                if UIQuestion::from(question.clone()) == ui_question.ui_question {
                    if ui_question.answer == question.answers[index] {
                        res = "true".to_string();
                        return Some((StatusCode::OK, res))
                    }
                }
                return None
            }).unwrap())
        },

        "hard" => {
            Ok(state.hard.iter().find_map(|question| {
                let index = question.correct_index;
                let mut res = to_string(&false).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

                if UIQuestion::from(question.clone()) == ui_question.ui_question {
                    if ui_question.answer == question.answers[index] {
                        res = "true".to_string();
                        return Some((StatusCode::OK, res))
                    }
                }
                return None
            }).unwrap())
        },

        "very_hard" => {
            Ok(state.very_hard.iter().find_map(|question| {
                let index = question.correct_index;
                let mut res = to_string(&false).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR).unwrap();

                if UIQuestion::from(question.clone()) == ui_question.ui_question {
                    if ui_question.answer == question.answers[index] {
                        res = "true".to_string();
                        return Some((StatusCode::OK, res))
                    }
                }
                return None
            }).unwrap())
        },
        _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
