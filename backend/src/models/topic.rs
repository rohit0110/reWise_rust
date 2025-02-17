use serde::{Serialize, Deserialize};
use crate::schema::topics;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::topics)]
pub struct Topic {
    pub id: i32,
    pub name: String,
    pub user_id: i32,
}
