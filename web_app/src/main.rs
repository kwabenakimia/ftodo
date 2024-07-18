#[macro_use] extern crate diesel;
extern crate dotenv;
mod config;

use actix_web::{App, HttpServer};
use actix_service::Service;
use actix_cors::Cors;
use actix_web::http::header;

mod schema;
mod database;
mod views;
mod to_do;
mod json_serialization;
mod jwt;
mod models;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
        //.allowed_origin("http://localhost:3000")
        .allowed_origin("http://localhost:3000")
        //.allowed_origin("http://127.0.0.1:3000")
        //.allowed_origin("https://automatic-space-umbrella-jggx9p6px57355qg-3000.app.github.dev")
        //.allowed_origin("*")
        .allowed_methods(vec!["GET", "POST"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        //.allowed_header(header::CONTENT_TYPE)
        .max_age(3600);
        // .allow_any_origin()
        // .allow_any_method()
        // .allow_any_header();
        let app = App::new()
            .wrap_fn(|req, srv|{
                //println!("{:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            }).configure(views::views_factory).wrap(cors);
        return app
    })
        .bind("127.0.0.1:8000")?
        .run()
        .await
}
