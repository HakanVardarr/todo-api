use super::*;

#[post("/register")]
pub async fn register(client: web::Data<Client>, user: web::Json<NewUser>) -> HttpResponse {
    let collection: Collection<User> = client.database("todo").collection("users");
    let password = hash_password(&user.password);

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
                .append_header(("Authorization", format!("Bearer: {token}")))
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

#[post("/login")]
pub async fn login(client: web::Data<Client>, login_user: web::Json<NewUser>) -> HttpResponse {
    let collection: Collection<User> = client.database("todo").collection("users");
    match find_user(collection, &login_user.username).await {
        Ok(user) => match verify_password(&user.password, &login_user.password) {
            Ok(verified) => {
                if verified {
                    let token = generate_jwt(user);
                    HttpResponse::Ok()
                        .append_header(("Authorization", format!("Bearer: {token}")))
                        .finish()
                } else {
                    HttpResponse::Unauthorized().body("Password is wrong")
                }
            }
            Err(_) => HttpResponse::InternalServerError().body("There is a error on server"),
        },
        Err(err) => match err {
            Error::NotFound => HttpResponse::NotFound().body("Cannot find user"),
            _ => HttpResponse::InternalServerError().body("There is a error on server"),
        },
    }
}
