mod role_select;
use crate::components::Icon;
use crate::server::{Role, User};
use leptos::*;
use role_select::RoleSelect;

#[component]
pub fn TbodyRow(user: User, roles_res: Resource<(), Vec<Role>>) -> impl IntoView {
    // удалить в бд, запросить пользователей, обновить ресурс или сигнал
    let on_click = |_: ev::MouseEvent| unimplemented!();

    view! {
        <tr class="flex">
        <td class="w-[172px] px-2.5">{user.login.clone()}</td>
        <td class="w-[213px] px-2.5">{user.registered_at.clone()}</td>
        <td class="flex w-[150px] px-2.5">
            <RoleSelect
                user_role_id={user.role_id}
                roles_res={roles_res}
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
