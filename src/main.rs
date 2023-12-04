#![warn(clippy::all)]
use warp::{http::Method, Filter};
mod routes;
mod store;
mod types;

#[tokio::main]
async fn main() {
    let store = store::Store::new("postgres://postgres@localhost:5432/rustwebdev").await;
    let store_filter = warp::any().map(move || store.clone()); // любой запрос будет работать с клоном Store

    let cors = warp::cors()
        .allow_any_origin() //влияет только на читаемость кода, без строки также удается зайти с иного сервера
        // .allow_origins(["http://localhost:3030", "http://127.0.0.1:3030"])
        .allow_header("content-type")
        .allow_methods(&[Method::PUT, Method::DELETE, Method::GET, Method::POST]);

    let get_questions = warp::get()
        .and(warp::path("questions"))
        .and(warp::path::end()) //модификатор предыдущего фильтра, без него он работал шире
        .and(warp::query()) //арг 1 get_questions: раст-тип по данным после ? в URL, в нашем случае хешмапа где -k,-v заданы: ?start=201&end=400
        .and(store_filter.clone()) //арг 2 get_questions: передается через фильтр, строку можно поместить выше соблюдая порядок аргументов
        .and_then(routes::question::get_questions);


    let add_question = warp::post()
        .and(warp::path("questions"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json()) //JSON в теле запроса десериализует в раст-тип в обработчике (структура Question)
        .and_then(routes::question::add_question);

    let update_question = warp::put()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>()) //похож на warp::query но часть до ?-знака
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(routes::question::update_question);

    let delete_question = warp::delete()
        .and(warp::path("questions"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(routes::question::delete_question);

    let add_answer = warp::post()
        .and(warp::path("answers"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::form()) //принять application/x-www-form-urlencoded формат в теле запроса и десериализовать в тип, указанный в обработчике (HashMap)
        .and_then(routes::answer::add_answer);

    let routes = get_questions
        .or(add_question)
        .or(update_question)
        .or(delete_question)
        .or(add_answer)
        .with(cors)
        .with(warp::trace::request())
        .recover(handle_errors::return_error); //тип Recover это структура с двумя полями: тип Filter и fn-обработчик ошибок, на текущем этапе никакой обработки не происходило это обертка

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await; // ему подойдет не только Recover
}

/*
localhost:3030/questions
localhost:3030/questions?start=1&end=200


подделать OPTIONS с другого сервера (только PowerShell)
Invoke-WebRequest -Uri "http://localhost:3030/questions" `
-Method OPTIONS `
-Headers @{
    "Access-Control-Request-Method" = "PUT"
    "Access-Control-Request-Headers" = "content-type"
    "Origin" = "https://not-origin.io"
} `
-Verbose


отправить POST (тест add_question())
Invoke-WebRequest -Uri "http://localhost:3030/questions" -Method POST -ContentType "application/json" -Body '{
    "id": "QI0002",
    "title": "New question",
    "content": "How does this work again?"
}'

отправить PUT
Invoke-WebRequest -Uri "http://localhost:3030/questions/QI0001" -Method PUT -ContentType "application/json" -Body '{
    "id": "QI0001",
    "title": "Updated question",
    "content": "OLD CONTENT"
}'


отправить DELETE
Invoke-WebRequest -Uri "http://localhost:3030/questions/QI0001" -Method DELETE -ContentType "application/json"
*/
