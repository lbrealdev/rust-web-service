use serde::{Deserialize, Serialize};

use crate::types::question::QuestionId;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Debug, Deserialize, Serialize, Eq, Hash, Clone, PartialEq)]
pub struct AnswerId(pub i32);

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NewAnswer {
    pub content: String,
    pub question_id: QuestionId,
}
