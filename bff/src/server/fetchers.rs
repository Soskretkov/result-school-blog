mod protected;
use super::types::db_interaction::{Comment, Post as DbPost};
use crate::server::error::Error;
use crate::server::types::export::PostWC;
use crate::store;
pub use protected::*;

pub async fn fetch_post(post_id: &str) -> Result<DbPost, Error> {
    let post_path_suffix = format!("posts/?id={post_id}");
    store::fetch::<Vec<DbPost>>(&post_path_suffix)
        .await
        .map_err(Error::Reqwest)?
        .into_iter()
        .next()
        .ok_or_else(|| Error::DbEntryNotFound)
}

pub async fn fetch_post_comments(post_id: &str) -> Result<Vec<Comment>, Error> {
    let comments_path_suffix = format!("comments/?post_id={}", post_id);
    store::fetch::<Vec<Comment>>(&comments_path_suffix)
        .await
        .map_err(Error::Reqwest)
}

pub async fn fetch_post_wc(post_id: &str) -> Result<PostWC, Error> {
    let post_path_suffix = format!("posts/?id={post_id}");
    let db_post = store::fetch::<Vec<DbPost>>(&post_path_suffix)
        .await
        .map_err(Error::Reqwest)?
        .into_iter()
        .next()
        .ok_or_else(|| Error::DbEntryNotFound)?;

    let comments_path_suffix = format!("comments/?post_id={}", post_id);
    let comments = store::fetch::<Vec<Comment>>(&comments_path_suffix)
        .await
        .map_err(Error::Reqwest)?;

    Ok(PostWC {
        id: db_post.id,
        title: db_post.title,
        image_url: db_post.image_url,
        content: db_post.content,
        created_at: db_post.created_at,
        comments,
    })
}
