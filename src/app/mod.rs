mod controllers;
mod middlewares;
mod queries;
mod repositories;
mod schemas;
mod services;

use crate::config::State;

pub fn init_app(state: State) -> tide::Server<()> {
    let mut app = tide::Server::new();

    app.with(middlewares::cors());

    app.at("/auth").nest(controllers::auth(state.clone()));
    app.at("/categories")
        .nest(controllers::category(state.clone()));

    app.at("/films").nest(controllers::films(state.clone()));

    app
}
