use crate::model::{NewTodo, UserWithId};
use actix_web::{
    delete, get, post,
    web::{self},
    HttpRequest, HttpResponse,
};
use error::*;
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::{bson::doc, options::FindOneAndUpdateOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::env;
use utils::*;

mod delete;
mod error;
mod get;
mod post;
mod utils;

pub use delete::*;
pub use get::*;
pub use post::*;
