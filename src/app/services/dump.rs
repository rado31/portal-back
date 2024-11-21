use crate::{
    app::schemas::{req, ChangesJSON, DeletedFilesJSON},
    config::State,
    utils::{copy_folder, dump_db, get_changes_json, set_changes_json},
};
use serde_json::json;
use std::{
    fs,
    io::{self, Write},
    path::Path,
};
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

    let folder = format!("{}/changes", body.folder);

    // 1. create 'changes' folder or recreate if exists
    if let Err(error) = fs::create_dir(&folder) {
        if error.kind() == io::ErrorKind::AlreadyExists {
            fs::remove_dir_all(&folder).unwrap();
            fs::create_dir(&folder).unwrap();
        } else {
            log::error!("Create folder on external memory: {error}");
            return Ok(Response::new(500));
        }
    }

    // 2. dump database and save in 'changes' folder
    if !dump_db(&req.state().db_uri, &format!("{folder}/dump.sql")) {
        log::error!("Couldn't dump database");
        return Ok(Response::new(500));
    };

    // 3. create 'static' folder (for movies, musics, books)
    for path in ["movies", "musics", "books"] {
        if let Err(error) = fs::create_dir(format!("{folder}/{path}")) {
            log::error!("Create {path} folder: {error}");
            return Ok(Response::new(500));
        };
    }

    // 4. read changes.json and copy files to own folders
    let json_file = get_changes_json();
    let upload_path = req.state().upload_path.clone();

    for id in json_file.movies {
        if let Err(error) = fs::create_dir(format!("{folder}/movies/{id}")) {
            log::error!("Create movie id folder: {error}");
            return Ok(Response::new(500));
        }

        if let Err(error) = copy_folder(
            Path::new(&format!("{upload_path}/movies/{id}")),
            Path::new(&format!("{folder}/movies/{id}")),
        ) {
            log::error!("Copy movie: {error}");
            return Ok(Response::new(500));
        }
    }

    if let Err(error) = copy_folder(
        Path::new(&format!("{upload_path}/musics")),
        Path::new(&format!("{folder}/musics")),
    ) {
        log::error!("Copy music: {error}");
        return Ok(Response::new(500));
    }

    if let Err(error) = copy_folder(
        Path::new(&format!("{upload_path}/books")),
        Path::new(&format!("{folder}/books")),
    ) {
        log::error!("Copy book: {error}");
        return Ok(Response::new(500));
    }

    // 5. create 'delete_files.json' and save it in 'changes' folder
    let json_string = serde_json::to_string_pretty(&json_file.deleted).unwrap();
    let mut file =
        fs::File::create(format!("{folder}/delete_files.json")).unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

    // 6. Reset 'changes.json' file
    let changes = ChangesJSON {
        movies: vec![],
        musics: vec![],
        books: vec![],
        deleted: DeletedFilesJSON {
            movies: vec![],
            musics: vec![],
            books: vec![],
        },
    };

    set_changes_json(&changes);

    Ok(Response::new(201))
}
