use mongodb::bson::Bson;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub todos: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserWithId {
    #[serde(rename = "_id")]
    pub id: Bson,
    pub username: String,
    pub password: String,
    pub todos: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct UserResponse {
    pub username: String,
    pub todos: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct NewTodo {
    pub content: String,
}
