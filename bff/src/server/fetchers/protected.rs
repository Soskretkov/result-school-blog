use crate::api_utils;
use crate::server::types::db_types::User as DbUser;
use crate::server::types::export_types::{Role, Session, User};
use gloo_timers::future::TimeoutFuture;

pub async fn fetch_all_users(session: &Session) -> Result<Vec<User>, String> {
    TimeoutFuture::new(1_000).await;
    let check_perm = |user: &DbUser| user.role_id.can_view_users();
    get_user_with_permission(session, check_perm).await?;
    Ok(api_utils::all_users().await)
}

pub async fn fetch_all_roles(session: &Session) -> Result<Vec<Role>, String> {
    TimeoutFuture::new(2_000).await;
    let check_perm = |user: &DbUser| user.role_id.can_view_roles();
    get_user_with_permission(session, check_perm).await?;
    Ok(api_utils::all_roles().await)
}

pub async fn fetch_user_by_id(session: &Session, id_to_find: &str) -> Result<Option<User>, String> {
    TimeoutFuture::new(1_000).await;
    let check_perm = |user: &DbUser| user.role_id.can_view_users();
    get_user_with_permission(session, check_perm).await?;
    Ok(api_utils::find_user_by_kv("id", id_to_find).await)
}

// Общая функция для получения пользователя и проверки его прав
async fn get_user_with_permission<F>(session: &Session, check_perm: F) -> Result<DbUser, String>
where
    F: FnOnce(&DbUser) -> bool,
{
    let user = api_utils::find_user_by_kv::<DbUser>("id", &session.user_id).await
        .ok_or_else(|| "Пользователь не существует".to_string())?;

    if check_perm(&user) {
        Ok(user)
    } else {
        Err("Недостаточно прав на операцию".to_string())
    }
}