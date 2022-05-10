use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use crate::schema::posts;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
pub struct Post {
    pub id: String,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostTitle {
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostBody {
    pub body: String,
}
