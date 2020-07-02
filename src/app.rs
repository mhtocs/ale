use super::config::Opt;
use tide::Server;

pub fn make_app(opt: Opt) -> Server<Opt> {
    let mut app = tide::with_state(opt);
    app.at("/").get(|_| async { Ok("OK") });
    app.at("/search").post(|_| async { Ok("OK") });
    app.at("/query").post(|_| async { Ok("OK") });
    app
}
