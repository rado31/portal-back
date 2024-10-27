use super::services;
use crate::config::State;

pub fn auth(state: State) -> tide::Server<State> {
    let mut api = tide::with_state(state);

    api.at("/me").get(services::get_me);
    api.at("/sign-in").post(services::sign_in);

    api
}
