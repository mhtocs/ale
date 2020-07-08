pub mod config;
mod handlers;
pub mod models;

use models::State;
use tide::Server;

pub fn make_app(state: State) -> Server<State> {
    let mut app = tide::with_state(state);
    app.at("/").post(|_| async { Ok("OK") });
    app.at("/search").post(handlers::search);
    app.at("/query").post(handlers::query);
    app
}
