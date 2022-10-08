extern crate rand;

use std::num::NonZeroU32;

use crate::db;
use crate::types::{self, APIResponse};
use actix_web::dev::Response;
use actix_web::web::Json;
use actix_web::{post, web, HttpRequest, HttpResponse, ResponseError};
use data_encoding::HEXUPPER;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::{thread_rng, Rng};
use ring::{digest, pbkdf2};
use sendgrid::SGClient;
use sendgrid::{Destination, Mail};
use std::env;

/* #[post("/login")]
async fn login(req: HttpRequest, info: Json<types::Users>) -> HttpResponse {
    // let password = &info.password;
    // let username = &info.email;
} */

pub fn create_salt() -> [u8; 16] {
    let rng = ring::rand::SystemRandom::new();
    let mut salt = [0u8; 16];
    thread_rng().try_fill(&mut salt).unwrap();
    return salt;
}

pub fn create_password(password: &str, salt: &[u8]) -> String {
    const CREDENTIAL_LEN: usize = digest::SHA512_OUTPUT_LEN;
    let mut hashed = [0u8; CREDENTIAL_LEN];
    let n_iter = NonZeroU32::new(8).unwrap();
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        n_iter,
        &salt,
        password.as_bytes(),
        &mut hashed,
    );
    let mut hashed_str = HEXUPPER.encode(&hashed);
    return hashed_str;
}

#[post("/create_user")]
async fn create(req: HttpRequest, info: Json<types::UserRequest>) -> HttpResponse {
    let salt = create_salt();
    let password = create_password(&info.password, &salt);
    let salt_str = HEXUPPER.encode(&salt);
    let ip = req.peer_addr().unwrap().to_string();
    let ip_split = ip.split(":").collect::<Vec<&str>>();

    let user = types::Users {
        id: 0,
        userid: salt_str.clone(),
        email: info.email.clone(),
        username: info.username.clone(),
        name: info.name.clone(),
        avatar_src: info.avatar_src.clone(),
        password: password,
        salt: salt_str.clone(),
        isgoogle: info.isgoogle,
        googleid: info.googleid.clone(),
        ip: ip_split[0].to_string(),
    };

    let client = db::BB8Pool.get().await;
    let query_result = client
        .get()
        .await
        .unwrap()
        .query(
            create_init_user_query,
            &[
                &user.email,
                &user.userid,
                &user.username,
                &user.name,
                &user.avatar_src,
                &user.password,
                &user.salt,
                &user.isgoogle,
                &user.googleid,
                &user.ip,
            ],
        )
        .await;
    let root = req.headers().get("origin").unwrap().to_str().unwrap();
    let message = format!(
        "Verify your account with Squawk.io. Please click the link {}/verify?{}",
        root, user.salt
    );
    let _ = match query_result {
        Ok(_result) => {
            let resp = APIResponse {
                result: "success",
                message: "User Creation Successful",
            };
            let send_grid_api = match env::var("SENDGRID_API_KEY") {
                Ok(v) => v,
                Err(e) => "None".to_string(),
            };
            let sendmail_result = actix_web::rt::task::spawn_blocking(move || {
                let sg = SGClient::new(send_grid_api);
                let mail_info = Mail::new()
                    .add_to(Destination {
                        address: &user.email,
                        name: &&user.name,
                    })
                    .add_from("eric.hwai.yu.yang@gmail.com")
                    .add_subject("Verify your account with Squawk.io")
                    .add_html(&message);
                return sg.send(mail_info);
            })
            .await;
            match sendmail_result {
                Err(err) => println!("Error: {}", err),
                Ok(body) => println!("Response: {:?}", body),
            };
            return HttpResponse::Ok().json(resp);
        }
        Err(e) => {
            let resp = APIResponse {
                result: "error",
                message: e.to_string(),
            };
            return HttpResponse::InternalServerError().json(resp);
        }
    };
}

pub static create_init_user_query: &str = "INSERT INTO users (email, userid, username, name, avatar_src, password, salt, isgoogle, googleid, ip) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id";
