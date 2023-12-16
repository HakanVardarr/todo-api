use todo_api::server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = Server::new("127.0.0.1:8080");
    server.run().await
}
