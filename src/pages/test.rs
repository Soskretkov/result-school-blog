use leptos::*;
use bff::bff_utils;


#[component]
pub fn Test() -> impl IntoView {
    let (click, set_click) = create_signal(0);
    let resource = create_resource(
        move || click, // Ресурс зависит от click
        |_| async move { bff_utils::test().await }
    );

    let on_click = move |_: ev::MouseEvent| {
        set_click.update(|r| *r += 1); // update уведомляет подписчиков даже если значение не менялось
    };

    view! {
        {move || {
            if true {
                match resource.get() {
                    None => view! { <p>"Loading..."</p> }.into_view(),
                    Some(data) => view! { <div>{data}</div> }.into_view(),
                }
            } else {
                view! { <p>"Loading..."</p> }.into_view()
            }
        }}

        <button on:click=on_click>
            "Загрузить"
        </button>
    }
}
