extern crate serde_derive;
use actix_session::Session;
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::http::header;
use actix_web::{web, App, HttpResponse, HttpServer};
use data_encoding::HEXUPPER;
use oauth2::basic::BasicClient;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    Scope, TokenUrl,
};
use serde_derive::Deserialize;
use std::env;

mod db;
mod types;
mod users;

async fn index(session: Session) -> HttpResponse {
    print!("{:?}", "I am Here");
    let mut resp = HttpResponse::Ok().body("Hello world!");
    return (resp);
}

#[actix_web::main]
async fn main() {
    let _server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .service(users::create)
    })
    .bind(("0.0.0.0", 4000))
    .expect("Error binding to port 4000")
    .run()
    .await;
}
