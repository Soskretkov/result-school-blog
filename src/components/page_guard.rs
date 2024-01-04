use leptos::*;

pub trait Protected {
    fn view(&self) -> View;
    fn can_access(&self) -> bool;
}

#[component]
pub fn PageGuard<T: Protected>(page: T) -> impl IntoView {
    if page.can_access() {
        page.view()
    } else {
        view! {
            <AccessDenied/>
        }
    }
}

#[component]
pub fn AccessDenied() -> impl IntoView {
    view!{<div>"Ошибка доступа"</div>}
}