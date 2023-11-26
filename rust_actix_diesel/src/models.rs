use diesel::{deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

use crate::schema::tweets;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Tweet {
    pub id: i32,
    pub message: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[table_name = "tweets"]
pub struct NewTweet<'a> {
    pub message: &'a str,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TweetPayload {
    pub message: String,
}
