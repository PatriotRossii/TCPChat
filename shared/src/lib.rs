use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    author: String,
    content: String,
}

impl ChatMessage {
    pub fn new<T1, T2>(author: T1, content: T2) -> Self
    where
        T1: Into<String>,
        T2: Into<String>,
    {
        Self {
            author: author.into(),
            content: content.into(),
        }
    }
}
