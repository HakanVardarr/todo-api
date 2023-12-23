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
                    // default settings are overly restrictive to reduce chance of
                    // misconfiguration leading to security concerns
                    Cors::default()
                        // add specific origin to allowed origin list
                        .allowed_origin("http://localhost:8080")
                        // allow any port on localhost
                        .allowed_origin_fn(|origin, _req_head| {
                            origin.as_bytes().starts_with(b"http://localhost")

                            // manual alternative:
                            // unwrapping is acceptable on the origin header since this function is
                            // only called when it exists
                            // req_head
                            //     .headers()
                            //     .get(header::ORIGIN)
                            //     .unwrap()
                            //     .as_bytes()
                            //     .starts_with(b"http://localhost")
                        })
                        // set allowed methods list
                        .allowed_methods(vec!["GET", "POST"])
                        // set allowed request header list
                        .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
                        // add header to allowed list
                        .allowed_header(header::CONTENT_TYPE)
                        // set list of headers that are safe to expose
                        .expose_headers(&[header::CONTENT_DISPOSITION])
                        // allow cURL/HTTPie from working without providing Origin headers
                        .block_on_origin_mismatch(false)
                        // set preflight cache TTL
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
