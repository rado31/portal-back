use crate::{app::schemas::req, config::State, utils::dump_db};
use async_std::{fs, io};
use serde_json::json;
use tide::{http::mime::JSON, Request, Response, Result};

pub async fn create(mut req: Request<State>) -> Result<Response> {
    let body: req::DumpBody = match req.body_json().await {
        Ok(val) => val,
        Err(error) => {
            let response = Response::builder(422)
                .body(json!({ "message": format!("{error}") }))
                .content_type(JSON)
                .build();

            return Ok(response);
        }
    };

    let folder = format!("{}/rado", body.folder);

    if let Err(error) = fs::create_dir(&folder).await {
        if error.kind() == io::ErrorKind::AlreadyExists {
            fs::remove_dir_all(&folder).await.unwrap();
            fs::create_dir(&folder).await.unwrap();
        } else {
            log::error!("Create folder on external memory: {error}");
            return Ok(Response::new(500));
        }
    }

    if !dump_db(&req.state().db_uri, &format!("{folder}/dump.sql")) {
        log::error!("Couldn't dump database");
        return Ok(Response::new(500));
    };

    Ok(Response::new(200))
}
