use crate::bff::server::User;
use crate::components::Icon;
use crate::types::RoleName;
use leptos::*;

#[component]
pub fn TbodyRow(user: User) -> impl IntoView {
    // удалить в бд, запросить пользователей, обновить ресурс или сигнал
    let on_click = |_: ev::MouseEvent| unimplemented!();

    view! {
        <tr class="flex">
        <td class="w-[172px] px-2.5">{user.login.clone()}</td>
        <td class="w-[213px] px-2.5">{user.registered_at.clone()}</td>
        <td class="flex w-[150px] px-2.5">
            <RoleSelect
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
pub fn RoleSelect(user_role_id: u8) -> impl IntoView {
    let option = |role_name: RoleName| {
        view! {
            <option value=role_name.as_u8() selected={user_role_id == role_name.as_u8()}>
                {role_name.as_str()}
            </option>
        }
    };

    view! {
        <select value={user_role_id}>
            { option(RoleName::Administrator) }
            { option(RoleName::Moderator) }
            { option(RoleName::Reader) }
        </select>
    }
}
