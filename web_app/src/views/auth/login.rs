
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
pub async fn login(credentials: web::Json<Login>, db: DB) ->  HttpResponse {
    let users = users::table
        .filter(&users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&db.connection).unwrap();

    if users.len() == 0 {
        return HttpResponse::NotFound().await.unwrap();
    } else if users.len() > 1 {
        return HttpResponse::Conflict().await.unwrap();
    }

    match users[0].clone().verify_password(credentials.password.clone()) {
        true => {
            let user_id = users[0].id;
            let token = JwToken::new(user_id);
            let raw_token = token.encode();
            let response = LoginResponse {
                token: raw_token.clone(),
            };
            let body = serde_json::to_string(&response).unwrap();
            HttpResponse::Ok().append_header(("token", raw_token)).json(&body)
        },
        false => HttpResponse::Unauthorized().finish() // converts HttpResponseBuilder to HttpResponse same as HttpResponse::Unauthorized().await.unwrap()
    }

}