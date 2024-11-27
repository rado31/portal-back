use crate::{app::schemas::ChangesJSON, config::State};
use async_std::{fs::OpenOptions, io};
use chrono::{Days, Duration, FixedOffset, Local, Utc};
use fern::Dispatch;
use jsonwebtoken::{
    decode, encode, errors::Result as JwtResult, Algorithm, DecodingKey,
    EncodingKey, Header, TokenData, Validation,
};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    io::Write,
    path::Path,
    process::{Command, Stdio},
};
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

    let result = String::from_utf8(res.stdout).unwrap();
    let result = result.trim();

    if result.ends_with(",") {
        return result.replace(",", "").parse().unwrap();
    }

    result.parse().unwrap()
}

pub fn check_media_password(
    booking_number: String,
    password: String,
    secret_key: String,
) -> bool {
    let pass_len = password.len();
    let last_digit = password.get((pass_len - 1)..).unwrap();

    for i in 0..2 {
        let timezone = FixedOffset::east_opt(5 * 3600).unwrap();
        let departure_time: String = Utc::now()
            .with_timezone(&timezone)
            .checked_add_days(Days::new(i))
            .unwrap()
            .to_string()
            .chars()
            .take(10)
            .collect();

        let data =
            format!("{booking_number}{last_digit}{departure_time}{secret_key}");

        let result = format!("{:x}", md5::compute(data));
        let re = regex::Regex::new(r"\D").unwrap();
        let cleaned = re.replace_all(&result, "").to_string();

        let mut pin: String = cleaned.chars().take(5).collect();
        pin += last_digit;

        if pin == password {
            return true;
        }
    }

    false
}

pub fn dump_db(db_uri: &str, path: &str) -> bool {
    Command::new("pg_dump")
        .args(["-d", db_uri, "-f", path])
        .status()
        .unwrap()
        .success()
}

pub fn copy_folder(src: &Path, dst: &Path) -> io::Result<()> {
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let dest_path = dst.join(entry.file_name());

        fs::copy(&path, &dest_path)?;
    }

    Ok(())
}

pub fn get_changes_json() -> ChangesJSON {
    let file = std::fs::File::open("changes.json").unwrap();
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

pub fn set_changes_json(file: &ChangesJSON) {
    let json_string = serde_json::to_string_pretty(file).unwrap();
    let mut file = std::fs::File::create("changes.json").unwrap();
    file.write_all(json_string.as_bytes()).unwrap();
}
