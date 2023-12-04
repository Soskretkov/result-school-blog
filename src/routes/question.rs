use std::collections::HashMap;
use tracing::{event, info, instrument, Level};
use warp::{http::StatusCode, Rejection, Reply};

use crate::store::Store;
use crate::types::pagination;
use crate::types::question::{NewQuestion, Question};
use pagination::Pagination;

#[instrument]
pub async fn get_questions(
    params: HashMap<String, String>, //это ?start=1&end=200 в адресе веб
    store: Store,
) -> Result<impl Reply, Rejection> {
    event!(target: "practical_rust_book", Level::INFO, "querying questions");
    let mut pagination = Pagination::default();

    // возврат JSON Array
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = pagination::extract_pagination(params)?;
    } else {
        info!(pagination = false);
    }

    let res: Vec<Question> = match store
        .get_questions(pagination.limit, pagination.offset)
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

pub async fn add_question(
    store: Store,
    new_question: NewQuestion,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.add_question(new_question).await {
        Ok(_) => Ok(warp::reply::with_status("Question added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn update_question(
    id: i32,
    store: Store,
    question: Question,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.update_question(question, id).await {
        Ok(res) => Ok(warp::reply::json(&res)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}

pub async fn delete_question(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.delete_question(id).await {
        Ok(_) => Ok(warp::reply::with_status(
            format!("Question {} deleted", id),
            StatusCode::OK,
        )),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
