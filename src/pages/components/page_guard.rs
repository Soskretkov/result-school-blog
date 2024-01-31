// назначение компонента: гарантировать children обновленного пользователя и роли
// похожий код в Header (login.rs)
use super::super::components::PageErrMsg;
use crate::types::GlobContext;
use crate::utils::is_sync_server_client_roles;
use leptos::*;

#[component]
pub fn PageGuard(children: ChildrenFn) -> impl IntoView {
    let glob_ctx = use_context::<GlobContext>().unwrap();
    let user_loading_signal = glob_ctx.user_resource.loading();
    let roles_pending_signal = glob_ctx.roles.pending();
    let children = store_value(children); // https://book.leptos.dev/interlude_projecting_children.html#solution

    view! { // внешний view чтобы отслеживался .with()
        {move || match glob_ctx.session.with(Option::is_some) {
            true => {
                if !user_loading_signal.get_untracked() {
                    glob_ctx.user_resource.refetch();
                } else {
                    logging::log!("PageGuard: оптимизация при авторизации");
                }

                if !roles_pending_signal.get_untracked() { glob_ctx.roles.dispatch(()); }

                view! {
                    <Show
                        when=move || !(roles_pending_signal.get() || user_loading_signal.get())
                        fallback=|| {
                            logging::log!("PageGuard: Show fallback: async ещё загружает данные");
                        }
                    >{
                        match is_sync_server_client_roles() {
                            true => match glob_ctx.roles.value().get_untracked() {
                                Some(Ok(_)) => {
                                    children.with_value(|children| children()).into_view()
                                    // children().into_view()
                                },
                                Some(Err(e)) => view! {<PageErrMsg err_msg={e}/>},
                                None => {
                                    logging::log!("PageGuard: неизвестная ошибка");
                                    view! {
                                        <PageErrMsg err_msg="Неизвестная ошибка".to_string()/>
                                    }
                                },
                            },
                            false => view! {
                                <PageErrMsg err_msg="Страница временно недоступна".to_string()/>
                            },
                        }
                    }</Show>
                }
            },
            false => view!{<PageErrMsg err_msg="Пользователь не авторизован".to_string()/>}.into_view(),
        }}
    }
}

// pub trait Protected {
//     fn view(&self) -> View;
//     fn can_access(&self) -> bool;
// }
// pub fn PageGuard<T: Protected + Copy + 'static>(page: T) -> impl IntoView {
