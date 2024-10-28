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

    api
}

pub fn films(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/")
        .get(services::get_films)
        .post(services::create_film);
    api.at("/:id").get(services::get_film);

    api
}
