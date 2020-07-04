use ale::{make_app, models::State};
use tide::http::{Request, Response, Url};

#[async_std::test]
async fn test_index() {
    let app = make_app(State { db_pool: None });
    let req = Request::post(Url::parse("http://localhost/").unwrap());
    let res: Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), 200);
}

#[async_std::test]
async fn test_search() {
    let app = make_app(State { db_pool: None });
    let req = Request::post(Url::parse("http://localhost/search").unwrap());
    let res: Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), 200);
}

#[async_std::test]
async fn test_query() {
    let app = make_app(State { db_pool: None });
    let req = Request::post(Url::parse("http://localhost/query").unwrap());
    let res: Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), 200);
}
