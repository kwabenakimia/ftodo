
use crate::diesel;
use crate::json_serialization::login_response::LoginResponse;
use crate::schema::users;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, Responder, HttpResponseBuilder};

use crate::database::DB;
use crate::models::user::user::User;
use crate::json_serialization::login::Login;
use crate::jwt::JwToken;

use std::collections::HashMap;


//pub async fn login() -> HttpResponse {
pub async fn login(credentials: web::Json<Login>, db: DB) -> impl Responder {
    //return HttpResponse::Ok();
    //  HttpResponse::Ok()
    // .content_type("text/html; charset=utf-8")
    // .body("<html>\
    //        <head></head> 
    //        <body> simple login view </body>
    //         </html>"
    // )
    // println!("##########################################");
    // println!("{:?}", credentials.username.clone());
    // println!("##########################################");
    let password = credentials.password.clone();
    let users = users::table
        .filter(&users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&db.connection).unwrap();

    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap();
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap();
    }

    match users[0].verify_password(password) {
        true => {
            let token = JwToken::new(users[0].id);
            let raw_token = token.encode();
            HttpResponse::Ok().append_header(("token", raw_token)).take().await.unwrap()
        },
        false => HttpResponse::Unauthorized().await.unwrap()
    }

}