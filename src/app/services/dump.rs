use crate::{
    app::schemas::{req, ChangesJSON, DeletedFilesJSON},
    config::State,
    utils::{copy_folder, dump_db},
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
    let file = match fs::File::open("changes.json") {
        Ok(f) => f,
        Err(error) => {
            log::error!("Read changes.json file: {error}");
            return Ok(Response::new(500));
        }
    };
    let reader = io::BufReader::new(file);
    let json_file: ChangesJSON = match serde_json::from_reader(reader) {
        Ok(f) => f,
        Err(error) => {
            log::error!("Serde changes.json file: {error}");
            return Ok(Response::new(500));
        }
    };

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

    for id in json_file.musics {
        if let Err(error) = fs::create_dir(format!("{folder}/musics/{id}")) {
            log::error!("Create music id folder: {error}");
            return Ok(Response::new(500));
        }

        if let Err(error) = copy_folder(
            Path::new(&format!("{upload_path}/musics/{id}")),
            Path::new(&format!("{folder}/musics/{id}")),
        ) {
            log::error!("Copy music: {error}");
            return Ok(Response::new(500));
        }
    }

    for id in json_file.books {
        if let Err(error) = fs::create_dir(format!("{folder}/books/{id}")) {
            log::error!("Create book id folder: {error}");
            return Ok(Response::new(500));
        }

        if let Err(error) = copy_folder(
            Path::new(&format!("{upload_path}/books/{id}")),
            Path::new(&format!("{folder}/books/{id}")),
        ) {
            log::error!("Copy book: {error}");
            return Ok(Response::new(500));
        }
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

    let json_string = serde_json::to_string_pretty(&changes).unwrap();
    let mut file = fs::File::create("changes.json").unwrap();
    file.write_all(json_string.as_bytes()).unwrap();

    Ok(Response::new(201))
}
