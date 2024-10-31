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

pub fn films(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/")
        .get(services::get_films)
        .post(services::create_film);
    api.at("/:id")
        .get(services::get_film)
        .post(services::upload_film);
    api.at("/video/:id").get(services::serve_film);
    api.at("/image/:id").post(services::upload_image);

    api
}
