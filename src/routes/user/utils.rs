use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

pub fn generate_jwt(user: User) -> String {
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

pub fn hash_password(password: &String) -> Option<String> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();
    match argon2.hash_password(password.as_bytes(), &salt) {
        Ok(pass) => Some(pass.to_string()),
        Err(_) => None,
    }
}

pub async fn find_user(collection: Collection<User>, username: &String) -> Result<User, Error> {
    match collection.find_one(doc! {"username": username}, None).await {
        Ok(user) => {
            if let Some(user) = user {
                Ok(user)
            } else {
                Err(Error::NotFound)
            }
        }
        Err(_) => Err(Error::InternalServerError),
    }
}

pub fn verify_password(password: &String, login_password: &String) -> Result<bool, Error> {
    let password_hash = PasswordHash::new(password);
    if let Ok(password) = password_hash {
        Ok(Argon2::default()
            .verify_password(login_password.as_bytes(), &password)
            .is_ok())
    } else {
        Err(Error::InternalServerError)
    }
}
