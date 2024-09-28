//! # quizzer
//! this module contains the logic for creating storing and reading quiz questions
//! from and to json files

use core::fmt;
use std::{
    error::Error,
    fs::File,
    io::{self, Read, Write},
    ops::Index,
    path::Path,
};

use serde::{Deserialize, Serialize};

/// quiz is a collection of multiple choice question
#[derive(Debug, Serialize, Deserialize)]
pub struct Quiz {
    questions: Vec<Question>,
}

#[derive(Debug, Clone)]
pub struct QuestionExistsError {}

impl Error for QuestionExistsError {}

impl fmt::Display for QuestionExistsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "can not add question to the quiz. question already exists"
        )
    }
}

impl Quiz {
    /// create a new Quiz
    pub fn new() -> Self {
        Quiz {
            questions: Vec::new(),
        }
    }

    /// add question to a quiz
    pub fn add_question(&mut self, q: Question) -> Result<(), QuestionExistsError> {
        if self.questions.contains(&q) {
            return Err(QuestionExistsError {});
        }
        self.questions.push(q);
        Ok(())
    }

    pub fn remove_question(&mut self, question_content: String) -> bool {
        let res = self
            .questions
            .iter()
            .position(|q| q.content == question_content);
        if let Some(index) = res {
            self.questions.remove(index);
            return true;
        }
        false
    }

    /// load a quiz from a file
    pub fn load(path: &Path) -> anyhow::Result<Quiz> {
        let mut buf = String::new();
        let mut file = File::open(path)?;
        file.read_to_string(&mut buf)?;
        let quiz: Quiz = serde_json::from_str(buf.as_str())?;
        Ok(quiz)
    }

    /// save quiz to a file
    pub fn save(&self, path: &Path) -> anyhow::Result<()> {
        let stringified = serde_json::to_string(self)?;
        let mut file = File::create(path)?;
        _ = file.write(stringified.as_bytes())?;
        Ok(())
    }
}

/// represent one answer to a question
#[derive(Debug, Serialize, Deserialize)]
pub struct Answer {
    /// the answer text
    content: String,

    /// indicate if the answer correct
    correct: bool,
}

impl Answer {
    pub fn new(content: String, correct: bool) -> Self {
        Answer { content, correct }
    }

    pub fn from_string(content: String) -> Self {
        Answer {
            content,
            correct: false,
        }
    }

    pub fn correct(&mut self, correct: bool) {
        self.correct = correct
    }
}

/// represent multiple choice question
#[derive(Debug, Serialize, Deserialize)]
pub struct Question {
    /// question text
    content: String,

    /// question answers. at least one of them should be valid.
    answers: Vec<Answer>,
}

impl PartialEq for Question {
    fn eq(&self, other: &Self) -> bool {
        self.content == other.content
    }
}

impl Question {
    /// create new question
    pub fn new(question: String) -> Self {
        Question {
            content: question,
            answers: Vec::new(),
        }
    }

    /// add answer
    pub fn add_answer(&mut self, answer: Answer) {
        self.answers.push(answer);
    }

    /// get question text
    pub fn content(&self) -> &String {
        &self.content
    }

    /// get question answers
    pub fn answers(&self) -> Vec<&str> {
        self.answers
            .iter()
            .map(|ans| ans.content.as_str())
            .collect::<Vec<&str>>()
    }

    pub fn is_valid(&self) -> bool {
        self.answers.iter().any(|ans| ans.correct)
    }

    /// get correct answers of the question
    pub fn correct_answers(&self) -> Vec<&str> {
        self.answers
            .iter()
            .filter(|ans| ans.correct)
            .map(|ans| ans.content.as_str())
            .collect::<Vec<&str>>()
    }

    /// check if answer is correct
    pub fn is_correct(&self, s: &str) -> bool {
        self.answers
            .iter()
            .filter(|ans| ans.correct)
            .any(|ans| ans.content.eq(s))
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn create_answer() {
        let ans = Answer::new("abc".to_string(), true);
        assert_eq!(ans.content.as_str(), "abc");
        assert!(ans.correct);

        let ans = Answer::from_string("abc".to_string());
        assert_eq!(ans.content.as_str(), "abc");
        assert!(!ans.correct);
    }

    #[test]
    fn create_question() {
        let text = "how old are you";
        let mut q = Question::new(text.to_string());
        q.add_answer(Answer::new("first answer".to_string(), false));
        q.add_answer(Answer::new("first answer".to_string(), false));
        q.add_answer(Answer::new("1".to_string(), true));
        assert!(q.is_valid());
        assert!(q.is_correct("1"));
    }
}
