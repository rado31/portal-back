use crate::{app::repositories, config::State};
use log::error;
use tide::{Body, Request, Response, Result};

pub async fn get_me(req: Request<State>) -> Result<Response> {
    let pool = req.state().pool.clone();

    match repositories::get_me(pool).await {
        Ok(admin) => {
            let mut res = Response::new(200);
            let body = Body::from_json(&admin).unwrap();
            res.set_body(body);
            Ok(res)
        }
        Err(error) => {
            error!("Get Me: {error}");
            Ok(Response::new(500))
        }
    }
}

pub async fn sign_in(_req: Request<State>) -> Result {
    Ok("".into())
}
