use crate::components::Icon;
use crate::types::outer_api::{Role, User};
use leptos::*;

// лайфтайм нужен потому как возвращаемыйimpl IntoView зависит от ссылки
#[component]
pub fn TableBodyRow<'a>(user: &'a User) -> impl IntoView {
    // удалить в бд, запросить пользователей, обновить ресурс или сигнал
    let on_click = |_: ev::MouseEvent| !unimplemented!();
    view! {
        <div class="flex">
            <div class="w-[172px] px-2.5">{user.login.clone()}</div>
            <div class="w-[213px] px-2.5">{user.registered_at.clone()}</div>
            <div class="flex w-auto px-2.5">
                <RoleSelect user_role_id={user.role_id}></RoleSelect>
                <Icon on:click=on_click id="fa-floppy-o" class="cursor-pointer text-[24px] ml-2.5"/>
            </div>
            <Icon on:click=on_click id="fa-trash-o" class="cursor-pointer text-[24px] ml-2.5"/>
        </div>
    }
}

#[component]
pub fn RoleSelect(user_role_id: u8) -> impl IntoView {
    view! {
        <select value={user_role_id}>
            <option value="">"Имя роли"</option>
        </select>
    }
}
