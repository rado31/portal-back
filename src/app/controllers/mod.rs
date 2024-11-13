use super::services;
use crate::config::State;

pub fn auth(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/sign-in").post(services::auth::sign_in);

    api
}

pub fn category(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    //api.with(middlewares::JwtMiddleware::new(state.key));
    api.at("/:id/sub").get(services::category::all);
    api.at("/sub/:id")
        .get(services::category::one)
        .delete(services::category::delete);
    api.at("/sub")
        .post(services::category::create)
        .put(services::category::update);

    api
}

pub fn movie(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/")
        .get(services::movie::all)
        .post(services::movie::create)
        .put(services::movie::update);
    api.at("/admin").get(services::movie::all_for_admin);
    api.at("/:id")
        .get(services::movie::one)
        .post(services::movie::upload)
        .delete(services::movie::delete);
    api.at("/sub/:id").get(services::movie::all_by_sc);
    api.at("/main-page").get(services::movie::main_page);
    api.at("/video/:id").get(services::movie::serve);
    api.at("/fraction/:id")
        .get(tide::sse::endpoint(services::movie::fraction));
    api.at("/image/:id").post(services::movie::upload_image);
    api.at("/search/:text").get(services::movie::search);

    api
}

pub fn music(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/")
        .get(services::music::all)
        .post(services::music::create)
        .put(services::music::update);
    api.at("/admin").get(services::music::all_for_admin);
    api.at("/:id")
        .get(services::music::one)
        .post(services::music::upload)
        .delete(services::music::delete);

    api
}

pub fn book(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/")
        .get(services::book::all)
        .post(services::book::create)
        .put(services::book::update);
    api.at("/admin").get(services::book::all_for_admin);
    api.at("/:id")
        .get(services::book::one)
        .post(services::book::upload)
        .delete(services::book::delete);

    api
}
