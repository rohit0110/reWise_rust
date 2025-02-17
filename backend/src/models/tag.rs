use serde::{Serialize, Deserialize};
use crate::schema::tags;

#[derive(Queryable, Insertable,Serialize, Deserialize)]
#[diesel(table_name = crate::schema::tags)]
pub struct Tag {
    pub id: i32,
    pub name: String,
}
