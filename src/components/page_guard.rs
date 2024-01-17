use leptos::*;
use crate::utils::isSyncServerClientRoles;

pub trait Protected {
    fn view(&self) -> View;
    fn can_access(&self) -> bool;
}

#[component]
pub fn PageGuard<T: Protected + 'static>(page: T) -> impl IntoView {
    let action = create_action(move |_: &()| async move {
        isSyncServerClientRoles().await
    });

    move || {
        action.dispatch(());

        action.value().get().map(|x| x).unwrap_or(false);

        if page.can_access() {
            page.view()
        } else {
            view! {
                <AccessDenied/>
            }
        }
    }
}

#[component]
pub fn AccessDenied() -> impl IntoView {
    view! {<div>"Ошибка доступа"</div>}
}
