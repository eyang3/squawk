extern crate rand;

use std::num::NonZeroU32;

use actix_web::dev::Response;
use actix_web::web::Json;
use actix_web::{post, HttpRequest, HttpResponse, ResponseError};
use data_encoding::HEXUPPER;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::{thread_rng, Rng};
use ring::{digest, pbkdf2};

use crate::db;
use crate::types::{self, APIResponse};

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
async fn create(req: HttpRequest, info: Json<types::Users>) -> HttpResponse {
    let mut client = db::POOL.get().unwrap();
    let salt = create_salt();
    let password = create_password(&info.password, &salt);
    let salt_str = HEXUPPER.encode(&salt);
    let user = types::Users {
        id: 0,
        email: info.email.clone(),
        userid: info.userid.clone(),
        username: info.username.clone(),
        name: info.name.clone(),
        avatar_src: info.avatar_src.clone(),
        password: password,
        salt: salt_str,
        isgoogle: info.isgoogle,
        googleid: info.googleid.clone(),
        ip: info.ip.clone(),
    };
    let query_result = client
        .query(
            "INSERT INTO users (email, userid, username, name, avatar_src, password, salt, isgoogle, googleid, ip) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING id",
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
            ]);
    let resp = match query_result {
        Ok(result) => {
            let resp = APIResponse {
                result: "success",
                message: "User Creation Successful",
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

/*
pub async fn create_init_user(user: types::Users) {
    let mut client = POOL.get().unwrap();
    let result = client.query(
        create_init_user_query,
        &[
            &user.email,
            &user.password,
            &user.isgoogle,
            &user.googleid,
            &user.ip,
        ],
    );
    return (result);
}

pub async fn login(user: types::Users) {
    let mut client = POOL.get().unwrap();
    let result = client.query(login_query, &[&user.email, &user.password]);
    return (result);
}
*/

pub static create_init_user_query: &str = "INSERT INTO users(email, password, salt, name, isgoogle, googleid, ip) VALUES ($1, $2, $3, $4, $5, $6, $7)";
