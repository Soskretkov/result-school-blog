use super::Content;
use leptos::*;

#[component]
pub fn PageErrMsg(children: ChildrenFn) -> impl IntoView {
    let children = store_value(children);
    view! {
        <Content header={"Ошибка"} >
            {children.with_value(|children| children())}
        </Content>
    }
}
