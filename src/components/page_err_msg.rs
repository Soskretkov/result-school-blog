use super::Content;
use leptos::*;

#[component]
pub fn PageErrMsg(err_msg: String) -> impl IntoView {
    view! {
        <Content header={"Ошибка"} >
            <div>{err_msg}</div>
        </Content>
    }
}
