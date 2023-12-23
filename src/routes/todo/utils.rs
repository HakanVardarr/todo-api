use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn get_jwt_from_header(req: &HttpRequest) -> Option<String> {
    if let Some(token) = req
        .headers()
        .get("Authorization")
        .and_then(|auth_header| auth_header.to_str().ok())
    {
        let token = token
            .split(":")
            .map(|s| s.to_string())
            .collect::<Vec<String>>();

        return Some(token[1].clone().trim().to_string());
    }

    None
}

pub fn decode_token_and_get_username(token: &str) -> Result<String, String> {
    let secret_key = env::var("SECRET_KEY").map_err(|_| "You need to set a secret key")?;
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_bytes()),
        &Validation::default(),
    )
    .map(|claims| claims.claims.sub)
    .map_err(|err| format!("Error decoding token: {}", err))
}

pub async fn find_user_by_username(
    client: &web::Data<Client>,
    username: &str,
) -> Result<UserWithId, Error> {
    let collection: Collection<UserWithId> = client.database("todo").collection("users");
    match collection
        .find_one(doc! { "username": username }, None)
        .await
    {
        Ok(user) => {
            if let Some(user) = user {
                Ok(user)
            } else {
                Err(Error::NotFound)
            }
        }
        Err(_) => Err(Error::NotFound),
    }
}

pub async fn update_user_todos(
    client: &web::Data<Client>,
    username: &str,
    new_todo_content: &String,
) -> Result<UserWithId, Error> {
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
        Ok(None) => Err(Error::NotFound),
        Err(err) => Err(Error::InternalServerError(err.to_string())),
    }
}

pub fn get_username_from_jwt(req: &HttpRequest) -> Result<String, Error> {
    if let Some(username) = req
        .headers()
        .get("Authorization")
        .and_then(|token| {
            let token = token.to_str().unwrap().to_string();
            let token = token
                .split(":")
                .map(|s| s.to_string())
                .collect::<Vec<String>>();

            decode::<Claims>(
                &token[1].trim(),
                &get_decoding_key(),
                &Validation::default(),
            )
            .ok()
        })
        .map(|decoded_token| decoded_token.claims.sub)
    {
        return Ok(username);
    } else {
        return Err(Error::Unauthorized);
    }
}

pub fn get_decoding_key() -> DecodingKey {
    let secret_key = env::var("SECRET_KEY").expect("You need to set secret key");
    DecodingKey::from_secret(secret_key.as_bytes())
}

pub async fn replace_user_todos(
    client: &web::Data<Client>,
    username: &str,
    new_todos: &Vec<String>,
) -> Result<UserWithId, Error> {
    let collection: Collection<UserWithId> = client.database("todo").collection("users");
    let update = doc! { "$set": { "todos": new_todos } };
    let options = FindOneAndUpdateOptions::builder()
        .return_document(mongodb::options::ReturnDocument::After)
        .build();

    match collection
        .find_one_and_update(doc! { "username": username }, update, options)
        .await
    {
        Ok(Some(user)) => Ok(user),
        Ok(None) => Err(Error::NotFound),
        Err(err) => Err(Error::InternalServerError(err.to_string())),
    }
}
