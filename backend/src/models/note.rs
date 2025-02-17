use serde::{Serialize, Deserialize};
use crate::schema::notes;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::notes)]
pub struct Note {
    pub id: i32,
    pub content: String,
    pub topic_id: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "notes"]
pub struct NewNote {
    pub content: String,
    pub topic_id: i32,
}
