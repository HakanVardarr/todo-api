use super::*;

#[delete("/todos/{index}")]
pub async fn delete_todo(
    client: web::Data<Client>,
    req: HttpRequest,
    index: web::Path<isize>,
) -> HttpResponse {
    if let Ok(username) = get_username_from_jwt(&req) {
        match find_user_by_username(&client, username.as_str()).await {
            Ok(user) => {
                let mut todos = user.todos;
                let index = index.into_inner();
                if index >= 0 && (index as usize) < todos.len() {
                    todos.remove(index as usize);
                    match replace_user_todos(&client, username.as_str(), &todos).await {
                        Ok(_) => HttpResponse::Ok().json(todos),
                        Err(_) => HttpResponse::InternalServerError().finish(),
                    }
                } else {
                    HttpResponse::BadRequest().finish()
                }
            }
            Err(_) => HttpResponse::NotFound().body("ERROR: User not found"),
        }
    } else {
        HttpResponse::Unauthorized().finish()
    }
}
