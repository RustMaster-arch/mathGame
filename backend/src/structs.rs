use serde::{Deserialize, Serialize};
#[derive(Default, Deserialize, Serialize)]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
    pub correct: String,
}
