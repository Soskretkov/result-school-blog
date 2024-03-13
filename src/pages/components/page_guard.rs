// назначение: гарантировать children обновленые и сверенные с сервером роли
use crate::components::PageErrMsg;
use crate::types::GlobContext;
use crate::utils::is_sync_server_client_roles;
use leptos::*;

#[component]
pub fn PageGuard(children: ChildrenFn) -> impl IntoView {
    let glob_ctx = use_context::<GlobContext>().unwrap();
    let roles_pending_signal = glob_ctx.roles.pending();
    let children = store_value(children); // https://book.leptos.dev/interlude_projecting_children.html#solution

    view! {
        {move || match glob_ctx.auth.get().and_then(|auth| auth.user) {
            Some(_user) => {
                if !roles_pending_signal.get_untracked() { glob_ctx.roles.dispatch(()); }

                view! {
                    <Show
                        when=move || !roles_pending_signal.get()
                        fallback=|| {
                            logging::log!("PageGuard: Show fallback: async ещё загружает роли");
                        }
                    >{
                        match is_sync_server_client_roles() {
                            true => match glob_ctx.roles.value().get_untracked() {
                                Some(Ok(_)) => {
                                    children.with_value(|children| children()).into_view()
                                },
                                Some(Err(e)) => {
                                    view! {<PageErrMsg>{e.to_owned()}</PageErrMsg>}
                                }
                                None => {
                                    logging::log!("PageGuard: неизвестная ошибка");
                                    view! {
                                        <PageErrMsg>"Неизвестная ошибка"</PageErrMsg>
                                    }
                                },
                            },
                            false => view! {
                                <PageErrMsg>"Страница временно недоступна"</PageErrMsg>
                            },
                        }
                    }</Show>
                }
            },
            None => view!{<PageErrMsg>"Пользователь не авторизован"</PageErrMsg>},
        }}
    }
}
