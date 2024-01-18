mod users_content;
use super::components::PageGuard;
use leptos::*;
use users_content::UsersContent;

#[component]
pub fn Users() -> impl IntoView {
    view! {
        <PageGuard>
            <UsersContent/>
        </PageGuard>
    }
}
