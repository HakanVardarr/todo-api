use todo_api::server::Server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = "0.0.0.0:8080";
    let server = Server::new(addr);
    server.run().await
}
