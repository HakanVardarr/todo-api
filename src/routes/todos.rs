use crate::model::{NewTodo, UserWithId};
use actix_web::{
    get, post,
    web::{self},
    HttpRequest, HttpResponse,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use mongodb::{bson::doc, options::FindOneAndUpdateOptions, Client, Collection};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

enum UserError {
    NotFound,
    InternalServerError(String),
    Unauthorized,
}

#[get("/todos")]
pub async fn get_todos(client: web::Data<Client>, req: HttpRequest) -> HttpResponse {
    if let Some(token) = get_jwt_from_cookie(&req) {
        match decode_token_and_get_username(&token) {
            Ok(username) => match find_user_by_username(&client, &username).await {
                Ok(Some(user)) => HttpResponse::Ok().json(user.todos),
                Ok(None) => HttpResponse::NotFound()
                    .body(format!("No user found with username {}", username)),
                Err(err) => HttpResponse::InternalServerError()
                    .body(format!("Error retrieving user: {}", err)),
            },
            Err(err) => HttpResponse::Unauthorized().body(format!("Error decoding token: {}", err)),
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
    if let Ok(username) = get_username_from_jwt(&req) {
        match update_user_todos(&client, &username, &new_todo.content).await {
            Ok(user) => HttpResponse::Ok().json(user.todos),
            Err(UserError::NotFound) => {
                HttpResponse::NotFound().body(format!("No user found with username {}", username))
            }
            Err(UserError::InternalServerError(err)) => {
                HttpResponse::InternalServerError().body(err)
            }
            Err(UserError::Unauthorized) => HttpResponse::Unauthorized().finish(),
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}

fn get_jwt_from_cookie(req: &HttpRequest) -> Option<String> {
    req.cookie("JWT").map(|cookie| cookie.value().to_string())
}

fn decode_token_and_get_username(token: &str) -> Result<String, String> {
    let secret_key = env::var("SECRET_KEY").map_err(|_| "You need to set a secret key")?;
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    )
    .map(|claims| claims.claims.sub)
    .map_err(|err| format!("Error decoding token: {}", err))
}

async fn find_user_by_username(
    client: &web::Data<Client>,
    username: &str,
) -> Result<Option<UserWithId>, String> {
    let collection: Collection<UserWithId> = client.database("todo").collection("users");
    match collection
        .find_one(doc! { "username": username }, None)
        .await
    {
        Ok(user) => Ok(user),
        Err(err) => Err(format!("Error retrieving user from the database: {}", err)),
    }
}

async fn update_user_todos(
    client: &web::Data<Client>,
    username: &str,
    new_todo_content: &String,
) -> Result<UserWithId, UserError> {
    let collection: Collection<UserWithId> = client.database("todo").collection("users");
    let update = doc! { "$push": { "todos": new_todo_content.clone() } };
    let options = FindOneAndUpdateOptions::builder()
        .return_document(mongodb::options::ReturnDocument::After)
        .build();

    match collection
        .find_one_and_update(doc! { "username": username }, update, options)
        .await
    {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(UserError::NotFound),
        Err(err) => Err(UserError::InternalServerError(err.to_string())),
    }
}

fn get_username_from_jwt(req: &HttpRequest) -> Result<String, UserError> {
    if let Some(username) = req
        .cookie("JWT")
        .and_then(|cookie| {
            decode::<Claims>(cookie.value(), &get_decoding_key(), &Validation::default()).ok()
        })
        .map(|decoded_token| decoded_token.claims.sub)
    {
        return Ok(username);
    } else {
        return Err(UserError::Unauthorized);
    }
}

fn get_decoding_key() -> DecodingKey {
    let secret_key = env::var("SECRET_KEY").expect("You need to set secret key");
    DecodingKey::from_secret(secret_key.as_bytes())
}
