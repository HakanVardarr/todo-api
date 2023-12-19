use crate::model::{NewUser, User};
use actix_web::{
    cookie::Cookie,
    post,
    web::{self},
    HttpResponse,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};
use error::*;
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};
use utils::*;

mod error;
mod post;
mod utils;

pub use post::*;
