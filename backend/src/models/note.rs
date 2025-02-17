use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Note {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_by: String,
}
