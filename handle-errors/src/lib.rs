use std::fmt;
use tracing::{event, instrument, Level};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    http::StatusCode,
    reject::Reject,
    Rejection,
};

#[derive(Debug)]
pub enum Error {
    ParseError(std::num::ParseIntError),
    MissingParameters,
    DatabaseQueryError,
}

impl Reject for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::ParseError(ref err) => write!(f, "Cannot parse parameter: {}", err),
            Error::MissingParameters => write!(f, "Missing parameter"),
            Error::DatabaseQueryError => write!(f, "Cannot update, invalid data."),
        }
    }
}
#[instrument]
pub async fn return_error(r: Rejection) -> Result<impl warp::Reply, warp::Rejection> {
    println!("{:?}", r); // не удалять, это не дебаг

    if let Some(crate::Error::DatabaseQueryError) = r.find() {
        let err_msg = crate::Error::DatabaseQueryError.to_string();
        event!(Level::ERROR, err_msg);
        Ok(warp::reply::with_status(
            err_msg,
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        event!(Level::ERROR, "CORS forbidden error: {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        event!(Level::ERROR, "Cannot deserizalize request body: {}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<Error>() {
        event!(Level::ERROR, "{}", error);
        Ok(warp::reply::with_status(
            error.to_string(),
            StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        event!(Level::WARN, "Requested route was not found");
        Ok(warp::reply::with_status(
            "Route not found!!!!!!!!!!!!!!".to_string(),
            StatusCode::NOT_FOUND,
        ))
    }
}
