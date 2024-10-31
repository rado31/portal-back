use super::services;
use crate::config::State;

pub fn auth(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/sign-in").post(services::sign_in);

    api
}

pub fn category(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    //api.with(middlewares::JwtMiddleware::new(state.key));
    api.at("/")
        .get(services::get_categories)
        .post(services::create_category);
    api.at("/:id").get(services::get_category);
    api.at("/:id/sub").get(services::get_sub_categories);
    api.at("/sub/:id")
        .get(services::get_sub_category)
        .delete(services::delete_sub_category);
    api.at("/sub")
        .post(services::create_sub_category)
        .put(services::update_sub_category);

    api
}

pub fn movie(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/")
        .get(services::get_movies)
        .post(services::create_movie);
    api.at("/:id")
        .get(services::get_movie)
        .post(services::upload_movie);
    api.at("/video/:id").get(services::serve_movie);
    api.at("/image/:id").post(services::upload_image);

    api
}
