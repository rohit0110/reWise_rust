use serde::Serialize;

#[derive(Serialize)]
pub struct TopicWithDetails {
    pub id: i32,
    pub name: String,
    pub notes: Vec<NoteDetails>,
    pub tags: Vec<TagDetails>,
}

#[derive(Serialize)]
pub struct NoteDetails {
    pub id: i32,
    pub heading: String,
    pub content: String,
}

#[derive(Serialize)]
pub struct TagDetails {
    pub id: i32,
    pub name: String,
}
