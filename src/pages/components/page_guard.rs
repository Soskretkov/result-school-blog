use super::PageErrMsg;
use crate::types::GlobContext;
use crate::utils::is_sync_server_client_roles;
use leptos::*;

#[component]
pub fn PageGuard(children: ChildrenFn) -> impl IntoView {
    let glob_ctx = use_context::<GlobContext>().unwrap();

    // https://book.leptos.dev/interlude_projecting_children.html#solution
    let children = store_value(children);

    // похожая логика в Header (login.rs)
    // внешний view чтобы отслеживался .get()
    view! {
        {move || match glob_ctx.auth.get() {
            Some(auth) => {
                view! {
                    <Transition
                        fallback=|| () // пользователь грузится
                    >{
                        // в учебном примере тоже вкладывается в замыкание
                        move || auth.user_resource.get().map(|user| {
                            glob_ctx.roles.dispatch(()); // обновляем роли (требуется сессия)
                            view! {
                                <Suspense
                                    fallback=|| () // роли грузятся
                                >{
                                    move || {
                                        match glob_ctx.roles.value().get() {
                                            Some(Ok(_)) => view! {
                                                {children.with_value(|children| children())}
                                            }.into_view(),
                                            Some(Err(e)) => view! {<PageErrMsg err_msg={e}/>},
                                            None => {
                                                logging::log!("PageGuard: неизвестная ошибка");
                                                view! {
                                                    <PageErrMsg err_msg="Неизвестная ошибка".to_string()/>
                                                }
                                            },
                                        }
                                    }
                                }</Suspense>
                            }
                        })
                    }</Transition>
                }
            }
            None => view!{<PageErrMsg err_msg="Пользователь не авторизован".to_string()/>},
        }}
    }
}

// pub trait Protected {
//     fn view(&self) -> View;
//     fn can_access(&self) -> bool;
// }
// pub fn PageGuard<T: Protected + Copy + 'static>(page: T) -> impl IntoView {