use super::*;

#[get("/todos")]
pub async fn get_todos(client: web::Data<Client>, req: HttpRequest) -> HttpResponse {
    if let Some(token) = get_jwt_from_header(&req) {
        match decode_token_and_get_username(&token) {
            Ok(username) => match find_user_by_username(&client, &username).await {
                Ok(user) => HttpResponse::Ok()
                    .append_header((
                        "Access-Control-Allow-Origin",
                        "https://todoapph.netlify.app",
                    ))
                    .json(user.todos),
                Err(_) => HttpResponse::NotFound().body("ERROR: User not found"),
            },
            Err(err) => HttpResponse::Unauthorized().body(format!("Error decoding token: {}", err)),
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
