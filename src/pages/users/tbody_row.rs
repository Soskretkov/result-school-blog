use crate::bff::server::{User, Role};
use crate::components::Icon;
use leptos::*;

#[component]
pub fn TbodyRow(user: User, roles: Resource<(), Vec<Role>>) -> impl IntoView {
    // удалить в бд, запросить пользователей, обновить ресурс или сигнал
    let on_click = |_: ev::MouseEvent| unimplemented!();

    view! {
        <tr class="flex">
        <td class="w-[172px] px-2.5">{user.login.clone()}</td>
        <td class="w-[213px] px-2.5">{user.registered_at.clone()}</td>
        <td class="flex w-[150px] px-2.5">
            <RoleSelect
                roles={roles}
                user_role_id={user.role_id}
            ></RoleSelect>
            <Icon
                on:click=on_click
                id="fa-floppy-o"
                class="cursor-pointer text-[24px] ml-2.5"
            />
        </td>
        <td class="w-auto">
            <Icon
                on:click=on_click
                id="fa-trash-o"
                class="cursor-pointer text-[24px] ml-2.5"
            />
        </td>
    </tr>
    }
}

#[component]
pub fn RoleSelect(roles: Resource<(), Vec<Role>>, user_role_id: u8) -> impl IntoView {
    view! {
        <select value={3}>
            <option value="">"Имя роли"</option>
        </select>
    }
}
