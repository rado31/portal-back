use crate::config::State;
use async_std::{fs::OpenOptions, io};
use chrono::{Duration, Local, Utc};
use fern::Dispatch;
use jsonwebtoken::{
    decode, encode, errors::Result as JwtResult, Algorithm, DecodingKey,
    EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::process::{Command, Stdio};
use tide::Request;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub is_admin: bool,
    pub exp: u64,
}

pub fn init_logger(release: bool) {
    let dispatch = Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .chain(std::io::stdout());

    let dispatch = if release {
        dispatch
            .level(log::LevelFilter::Error)
            .chain(fern::log_file("output.log").unwrap())
    } else {
        dispatch.level(log::LevelFilter::Debug)
    };

    dispatch.apply().unwrap();
}

pub fn create_token(is_admin: bool, exp: u64, secret_key: &str) -> String {
    let exp_time = Utc::now() + Duration::seconds(exp as i64);

    encode(
        &Header::default(),
        &Claims {
            is_admin,
            exp: exp_time.timestamp() as u64,
        },
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .unwrap()
}

pub fn verify_token(
    token: &str,
    secret_key: &str,
) -> JwtResult<TokenData<Claims>> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 0; // additional seconds

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &validation,
    )
}

pub async fn save_file(path: String, req: Request<State>) -> io::Result<u64> {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .await
        .unwrap();

    Ok(io::copy(req, file).await?)
}

pub fn count_total_frames(path: &str) -> i32 {
    let res = Command::new("ffprobe")
        .args([
            "-v",
            "error",
            "-select_streams",
            "v:0",
            "-count_packets",
            "-show_entries",
            "stream=nb_read_packets",
            "-of",
            "csv=p=0",
            path,
        ])
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    String::from_utf8(res.stdout)
        .unwrap()
        .trim()
        .parse()
        .unwrap()
}
