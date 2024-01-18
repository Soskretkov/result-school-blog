use leptos::*;
use crate::utils::isSyncServerClientRoles;
use crate::types::GlobContext;

pub trait Protected {
    fn view(&self) -> View;
    fn can_access(&self) -> bool;
}

#[component]
pub fn PageGuard<T: Protected + Copy + 'static>(page: T) -> impl IntoView {
    view! {
        <Suspense
            fallback=|| () // пользователь грузится
        >{
            move || {
                let glob_ctx = use_context::<GlobContext>().unwrap();
                glob_ctx.user_info.track();
                match glob_ctx.user_info.user_data() {
                    Some(_) => {
                        glob_ctx.roles.dispatch(()); // получаем роли (требуется сессия)
                        view! {
                            <Suspense
                            fallback=|| () // роли грузятся
                        >{
                            move || {
                                match glob_ctx.roles.value().get() {
                                    Some(Ok(_)) => {
                                        page.view()
                                    },
                                    Some(Err(e)) => {
                                        view! {
                                            <AccessDenied err_msg={e}/>
                                        }
                                    },
                                    None => view! {
                                        <AccessDenied err_msg="Неизвестная ошибка".to_string()/>
                                    },
                                }
                            }
                        }</Suspense>
                        }
                    },
                    None => view! {
                        <AccessDenied err_msg="Войдите в систему".to_string()/>
                    },
                }
            }
        }</Suspense>
    }
}

#[component]
pub fn AccessDenied(err_msg: String) -> impl IntoView {
    view! {<div>{err_msg}</div>}
}
