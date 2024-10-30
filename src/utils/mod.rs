use async_std::{fs, process::Command};
use chrono::{Duration, Local, Utc};
use fern::Dispatch;
use jsonwebtoken::{
    decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: u64,
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

pub fn create_token(exp: u64, secret_key: &str) -> String {
    let exp_time = Utc::now() + Duration::seconds(exp as i64);

    encode(
        &Header::default(),
        &Claims {
            exp: exp_time.timestamp() as u64,
        },
        &EncodingKey::from_secret(secret_key.as_ref()),
    )
    .unwrap()
}

pub fn verify_token(token: &str, secret_key: &str) -> bool {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.leeway = 0; // additional seconds

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret_key.as_ref()),
        &validation,
    ) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn create_folder(path: &str) -> Result<(), std::io::Error> {
    match fs::create_dir(path).await {
        Ok(_) => Ok(()),
        Err(error) => {
            log::error!("Create folder: {error}");
            Err(error)
        }
    }
}

pub async fn fraction_video(video_path: &str, output_path: &str) {
    Command::new("ffmpeg")
        .args(["-i", video_path, "-map", "0", "-f", "dash", output_path])
        .output()
        .await
        .unwrap();
}
