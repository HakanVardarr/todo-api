use todo_api::server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "127.0.0.1:8080";
    let server = Server::new(addr);
    server.run().await
}
