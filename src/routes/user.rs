use crate::model::{NewTodo, NewUser, User, UserWithId};
use actix_web::{
    cookie::Cookie,
    get, post,
    web::{self},
    HttpRequest, HttpResponse,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use mongodb::{bson::doc, options::FindOneAndUpdateOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};

#[get("/todos")]
pub async fn get_todos(client: web::Data<Client>, req: HttpRequest) -> HttpResponse {
    let secret_key = env::var("SECRET_KEY").expect("You need to set secret key");
    if let Some(cookie) = req.cookie("JWT") {
        let token = cookie.value();
        let decoded_token_claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret_key.as_bytes()),
            &Validation::default(),
        )
        .unwrap();

        let username = decoded_token_claims.claims.sub;
        let collection: Collection<UserWithId> = client.database("todo").collection("users");

        match collection
            .find_one(doc! { "username": &username }, None)
            .await
        {
            Ok(Some(user)) => HttpResponse::Ok().json(user.todos),
            Ok(None) => {
                HttpResponse::NotFound().body(format!("No user found with username {username}"))
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

#[post("/todos")]
pub async fn post_todo(
    client: web::Data<Client>,
    req: HttpRequest,
    new_todo: web::Form<NewTodo>,
) -> HttpResponse {
    let secret_key = env::var("SECRET_KEY").expect("You need to set secret key");
    if let Some(cookie) = req.cookie("JWT") {
        let token = cookie.value();
        let decoded_token_claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(secret_key.as_bytes()),
            &Validation::default(),
        )
        .unwrap();

        let username = decoded_token_claims.claims.sub;
        let collection: Collection<UserWithId> = client.database("todo").collection("users");
        let update = doc! { "$push": { "todos": new_todo.content.clone() } };

        let options = FindOneAndUpdateOptions::builder()
            .return_document(mongodb::options::ReturnDocument::After)
            .build();

        match collection
            .find_one_and_update(doc! { "username": &username }, update, options)
            .await
        {
            Ok(Some(user)) => HttpResponse::Ok().json(user.todos),
            Ok(None) => {
                HttpResponse::NotFound().body(format!("No user found with username {username}"))
            }
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

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
