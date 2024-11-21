mod controllers;
mod middlewares;
mod queries;
mod repositories;
pub mod schemas;
mod services;

use crate::config::State;

pub fn init_app(state: State) -> tide::Server<()> {
    let mut app = tide::Server::new();

    app.with(middlewares::cors());

    app.at("/auth").nest(controllers::auth(state.clone()));
    app.at("/categories")
        .nest(controllers::category(state.clone()));

    app.at("/movies").nest(controllers::movie(state.clone()));
    app.at("/musics").nest(controllers::music(state.clone()));
    app.at("/books").nest(controllers::book(state.clone()));
    app.at("/dump").nest(controllers::dump(state.clone()));
    app.at("/uploads").serve_dir("uploads").unwrap();

    app
}
