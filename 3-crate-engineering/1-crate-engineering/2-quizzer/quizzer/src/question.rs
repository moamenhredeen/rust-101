use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    pub content: String,
    pub correct_answer: String,
    pub incorrect_answers: Vec<String>,
}
