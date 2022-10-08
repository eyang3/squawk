use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/*
    id serial primary key,
    email text unique,
    userid text unique,
    username text,
    name text,
    avatar_src text,
    password varchar(128),
    salt varchar(32),
    isgoogle boolean,
    googleid text,
    ip varchar(16)
*/

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Users {
    pub id: i32,
    pub userid: String,
    pub email: String,
    pub username: String,
    pub name: String,
    pub avatar_src: String,
    pub password: String,
    pub salt: String,
    pub isgoogle: bool,
    pub googleid: String,
    pub ip: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserRequest {
    pub email: String,
    pub username: String,
    pub name: String,
    pub avatar_src: String,
    pub password: String,
    pub isgoogle: bool,
    pub googleid: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct APIResponse<P> {
    pub result: &'static str,
    pub message: P,
}
