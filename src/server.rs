use crate::model::User;
use crate::routes::*;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::{middleware::Logger, services, web, App, HttpServer};
use dotenv::dotenv;
use mongodb::{bson::doc, options::IndexOptions, Client, IndexModel};
use std::env;

pub struct Server<'a> {
    addr: &'a str,
}

impl<'a> Server<'a> {
    pub fn new(addr: &'a str) -> Self {
        Self { addr }
    }
    async fn create_username_index(client: &Client) {
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! { "username": 1 })
            .options(options)
            .build();
        client
            .database("todo")
            .collection::<User>("users")
            .create_index(model, None)
            .await
            .expect("creating an index should succeed");
    }
    pub async fn run(&self) -> std::io::Result<()> {
        dotenv().ok();
        env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
        let uri = env::var("URI").expect("You need to set URI.");
        let client = Client::with_uri_str(uri.clone())
            .await
            .expect("Failed to connect.");

        Server::create_username_index(&client).await;

        HttpServer::new(move || {
            App::new()
                .wrap(
                    Cors::default()
                        .allowed_origin("https://todoapph.netlify.app")
                        .allow_any_method()
                        .allow_any_header()
                        .expose_any_header()
                        .max_age(3600),
                )
                .wrap(Logger::new(r#"%a "%r" %s %T"#))
                .app_data(web::Data::new(client.clone()))
                .service(services![
                    healthcheck,
                    get_todos,
                    post_todo,
                    delete_todo,
                    register,
                    login
                ])
        })
        .bind(self.addr)?
        .run()
        .await
    }
}
