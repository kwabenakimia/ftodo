#[macro_use]
extern crate diesel;
extern crate dotenv;
mod config;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::http::header;
use actix_web::{App, HttpResponse, HttpServer, middleware::Logger};
use futures::future::{ok, Either};
mod counter;

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
    let site_counter = counter::Counter{value: 0};

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    println!("main.rs::header::CONTENT_TYPE=: {:?}", header::CONTENT_TYPE);

    HttpServer::new(|| {
        // let cors = Cors::default()
        //     .allowed_origin("http://localhost:3000")
        //     .allowed_methods(vec!["GET", "POST"])
        //     .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        //     .allowed_header(header::X_CONTENT_TYPE_OPTIONS)
        //     .allowed_header(header::CONTENT_TYPE)
            //.allowed_header(header::CONTENT_TYPE)
            //.allow_any_origin()
            //.allow_any_method()
            //.allow_any_header()
            //.max_age(3600);
        //let cors = Cors::permissive();
        let cors = Cors::permissive()
            .supports_credentials();
            //.allowed_header(header::X_CONTENT_TYPE_OPTIONS);
        // let site_counter = counter::Counter{value: 0};
        // site_counter.save();

       //const outcome : &str = "test";

        let app = App::new()
            .wrap_fn(|req, srv| {
                //println!("{}", outcome);
                let passed: bool;

                // let mut site_counter = counter::Counter::load().unwrap();   
                // site_counter.value += 1;
                // println!("{:?}", &site_counter);
                // site_counter.save();

                if req.path().contains(&format!("/{}", ALLOWED_VERSION)) {
                    passed = true;
                } else {
                    passed = false;
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
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"));
        return app;
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
