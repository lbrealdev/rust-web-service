use serde::{Deserialize, Serialize};

// Adding the Clone trait which we use in the
// get_questions function further down
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Question {
    pub id: QuestionId,
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Eq, Hash, Clone, PartialEq)]
pub struct QuestionId(pub i32);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateQuestion {
    pub title: String,
    pub content: String,
    pub tags: Option<Vec<String>>,
}
