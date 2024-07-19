#[macro_use]
extern crate diesel;
extern crate dotenv;
mod config;

use std::sync::Arc;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::http::header;
use actix_web::{App, HttpResponse, HttpServer};
use diesel::result;
use futures::future::{ok, Either};

mod database;
mod json_serialization;
mod jwt;
mod models;
mod schema;
mod to_do;
mod views;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    const ALLOWED_VERSION: &'static str = include_str!("./output_data.txt");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            //.allowed_origin("http://localhost:3000")
            //.allowed_origin("http://127.0.0.1:3000")
            //.allowed_origin("https://automatic-space-umbrella-jggx9p6px57355qg-3000.app.github.dev")
            //.allowed_origin("*")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .max_age(3600);
        // .allow_any_origin()
        // .allow_any_method()
        // .allow_any_header();
        let app = App::new()
            .wrap_fn(|req, srv| {
                let passed: bool;
                if req.path().contains(&format!("/{}", ALLOWED_VERSION)) {
                    passed = false;
                } else {
                    passed = true;
                }

                let end_result = match passed {
                    true => Either::Left(srv.call(req)),
                    false => {
                        let resp = HttpResponse::NotImplemented().body(format!("only {} API is supported", ALLOWED_VERSION));
                        Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                    }
                };
                async move {
                    let result = end_result.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
            .wrap(cors);
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
