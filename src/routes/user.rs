use crate::model::{NewUser, User};
use actix_web::{
    cookie::Cookie,
    post,
    web::{self},
    HttpResponse,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::{bson::doc, Client, Collection};
use serde::{Deserialize, Serialize};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

fn hash_password(password: String) -> Option<String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(pass) => Some(pass.to_string()),
        Err(_) => None,
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

fn generate_jwt(user: User) -> String {
    let secret_key = env::var("SECRET_KEY").expect("You need to set secret key");
    let expiration_time = SystemTime::now()
        .checked_add(std::time::Duration::from_secs(86400))
        .expect("System time overflow");

    let exp_timestamp = expiration_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as usize;

    let claims = Claims {
        sub: user.username.clone(),
        exp: exp_timestamp,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret_key.as_bytes()),
    )
    .expect("Cannot encode claims.");

    token
}

#[post("/register")]
pub async fn register(client: web::Data<Client>, user: web::Form<NewUser>) -> HttpResponse {
    let collection: Collection<User> = client.database("todo").collection("users");
    let password = hash_password(user.password.clone());
    if password.is_none() {
        return HttpResponse::InternalServerError().finish();
    }
    let new_user = User {
        username: user.username.clone(),
        password: password.unwrap(),
        todos: vec![],
    };

    match collection.insert_one(new_user.clone(), None).await {
        Ok(_) => {
            let token = generate_jwt(new_user);
            HttpResponse::Ok()
                .cookie(
                    Cookie::build("JWT", token)
                        .max_age(time::Duration::seconds(86400))
                        .http_only(true)
                        .secure(true)
                        .finish(),
                )
                .finish()
        }
        Err(err) => {
            let error_kind = err.kind.as_ref();

            match error_kind {
                mongodb::error::ErrorKind::Write(_) => {
                    HttpResponse::Conflict().body("User Already Exists")
                }
                _ => HttpResponse::InternalServerError().body(err.to_string()),
            }
        }
    }
}
