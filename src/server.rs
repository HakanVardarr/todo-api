use crate::model::User;
use crate::routes::*;
use actix_web::{web, App, HttpServer};
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
        let uri = env::var("URI").expect("You need to set URI.");
        let client = Client::with_uri_str(uri).await.expect("Failed to connect.");

        Server::create_username_index(&client).await;

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(client.clone()))
                .service(healthcheck)
                .service(get_user)
        })
        .bind(self.addr)?
        .run()
        .await
    }
}
