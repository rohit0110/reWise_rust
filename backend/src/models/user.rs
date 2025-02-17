use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String, // This needs to match the Nullable timestamp
}
