use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Default, Debug, PartialEq)]
pub struct UIQuestion {
    pub question: String,
    pub answers: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct Question {
    pub question: String,
    pub answers: Vec<String>,
    pub correct_index: usize,
}

#[derive(Clone, Serialize, Deserialize, Default, Debug)]
pub struct ClientRequest {
    pub ui_question: UIQuestion,
    pub difficulty: String,
    pub answer: String,
}

impl Question {
    pub fn new(question: &str, answers: Vec<&str>, correct: usize) -> Self {
        let new = answers.iter().map(|answer| answer.to_string()).collect();
        Self { question: question.to_string(), answers: new, correct_index: correct }
    }
}

impl From<Question> for UIQuestion {
    fn from(value: Question) -> Self {
        UIQuestion{question: value.question, answers: value.answers}
    }
}
