use ale::{make_app, models::*};
use chrono::{DateTime, NaiveDateTime, Utc};
use tide::http::{Request, Response, Url};
use tide::Body;

// hack for logging
//use std::sync::Once;
//static START: Once = Once::new();

#[async_std::test]
async fn test_index() {
    /*
    START.call_once(|| {
        pretty_env_logger::init();
    });*/

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
    pretty_env_logger::init();
    let app = make_app(State { db_pool: None });

    let query = Query {
        range: Range {
            from: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc),
            to: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc),
        },
        interval_ms: 23232,
        max_data_points: 32212,
        targets: vec![Target {
            target: None,
            ref_id: "".to_string(),
            _type: "".to_string(),
        }],
    };

    let mut req = Request::post(Url::parse("http://localhost/query").unwrap());
    let body = Body::from_json(&query).unwrap();
    req.set_body(body);
    // let query: Query = body.into_json().await.unwrap();
    let res: Response = app.respond(req).await.unwrap();
    assert_eq!(res.status(), 200);
}

// a sample test
#[async_std::test]
async fn test_sample() {
    //  pretty_env_logger::init();

    let query = Query {
        range: Range {
            from: DateTime::<Utc>::from(chrono::Local::now()),
            to: DateTime::<Utc>::from(chrono::Local::now()),
        },
        interval_ms: 23232,
        max_data_points: 32212,
        targets: vec![Target {
            target: None,
            ref_id: "".to_string(),
            _type: "".to_string(),
        }],
    };

    dbg!(&query); //cargo test -- --nocapture

    let body = Body::from_json(&query).unwrap();

    let cat: Query = body.into_json().await.unwrap();
    assert_eq!(cat.interval_ms, 23232);
}
