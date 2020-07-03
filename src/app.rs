use super::config::Opt;
use super::handlers;
use tide::Server;

pub fn make_app(opt: Opt) -> Server<Opt> {
    let mut app = tide::with_state(opt);
    app.at("/").post(|_| async { Ok("OK") });
    app.at("/search").post(handlers::search);
    app.at("/query").post(handlers::query);
    app
}
