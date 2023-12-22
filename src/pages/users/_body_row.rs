use crate::types::outer_api::User;
use leptos::*;

async fn asn() -> String {
    "я асинхронная функция".to_string()
}

#[component]
pub fn BodyRow<'a>(user: &'a User) -> impl IntoView {
    // let once = create_resource(|| (), |_| async move { asn().await });
    let once = create_resource(|| (), |_| async move { bff::bff_utils::all_users::<User>().await });

    view! {
        {move || match once.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) => view! { <div>{data.into_iter().next().unwrap().login}</div> }.into_view()
            // Some(data) => view! { <div>{data}</div> }.into_view()
        }}
    }
}
