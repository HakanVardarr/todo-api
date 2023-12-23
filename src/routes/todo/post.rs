use super::*;

#[post("/todos")]
pub async fn post_todo(
    client: web::Data<Client>,
    req: HttpRequest,
    new_todo: web::Json<NewTodo>,
) -> HttpResponse {
    if let Ok(username) = get_username_from_jwt(&req) {
        match update_user_todos(&client, &username, &new_todo.content).await {
            Ok(user) => HttpResponse::Ok().json(user.todos),
            Err(Error::NotFound) => {
                HttpResponse::NotFound().body(format!("No user found with username {}", username))
            }
            Err(Error::InternalServerError(err)) => HttpResponse::InternalServerError().body(err),
            Err(Error::Unauthorized) => HttpResponse::Unauthorized().finish(),
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
