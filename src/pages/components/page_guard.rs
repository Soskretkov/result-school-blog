use crate::types::GlobContext;
use super::PageErrMsg;
use crate::utils::is_sync_server_client_roles;
use leptos::*;

// pub trait Protected {
//     fn view(&self) -> View;
//     fn can_access(&self) -> bool;
// }
// pub fn PageGuard<T: Protected + Copy + 'static>(page: T) -> impl IntoView {

#[component]
pub fn PageGuard(children: ChildrenFn) -> impl IntoView {
    // https://book.leptos.dev/interlude_projecting_children.html#solution
    let children = store_value(children);

    view! {
        <Suspense
            fallback=|| () // пользователь грузится
        >{
            move || {
                let glob_ctx = use_context::<GlobContext>().unwrap();
                match glob_ctx.user_info.user_data() {
                    Some(_) => {
                        logging::log!("page_guard.rs: данные пользователя загружены - {}", glob_ctx.user_info.is_loaded());
                        glob_ctx.roles.dispatch(()); // получаем роли (требуется сессия)
                        view! {
                            <Suspense
                                fallback=|| () // роли грузятся
                            >{
                                move || {
                                    match glob_ctx.roles.value().get() {
                                        Some(Ok(_)) => view! {
                                            {children.with_value(|children| children())}
                                        }.into_view(),
                                        Some(Err(e)) =>
                                            view! {
                                                <PageErrMsg err_msg={e}/>
                                            },
                                        None => view! {
                                            <PageErrMsg err_msg="Неизвестная ошибка".to_string()/>
                                        },
                                    }
                                }
                            }</Suspense>
                        }
                    }.into_view(),
                    None => view! {
                        <PageErrMsg err_msg="Пользователь не авторизован".to_string()/>
                    },
                }
            }
        }</Suspense>
    }
}
