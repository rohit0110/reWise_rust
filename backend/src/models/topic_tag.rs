use serde::{Serialize, Deserialize};
use crate::schema::topic_tags;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::topic_tags)]
pub struct TopicTag {
    pub topic_id: i32,
    pub tag_id: i32,
}