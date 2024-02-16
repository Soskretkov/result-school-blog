mod role_select;
mod save_icon;
use crate::components::Icon;
use crate::server::User;
use leptos::*;
use role_select::RoleSelect;
use save_icon::SaveIcon;

#[component]
pub fn TbodyRow(user: User) -> impl IntoView {
    // удалить в бд, запросить пользователей, обновить ресурс или сигнал
    let on_click = |_: ev::MouseEvent| unimplemented!();

    let (new_role_type, set_new_role_type) = create_signal(user.role_id);

    view! {
        <tr class="flex mt-2.5 ">
            <td class="w-[170px] px-2.5 flex items-center">{user.login.clone()}</td>
            <td class="w-[170px] px-2.5 flex items-center">{user.registered_at.clone()}</td>
            <td class="w-auto flex items-center justify-between">
                <RoleSelect
                initial_role_type=user.role_id
                set_new_role_type = set_new_role_type
                />
                {move || view! {
                    <SaveIcon is_selected = user.role_id == new_role_type.get() />
                }}
            </td>
            <td class="w-auto flex items-center">
                <Icon
                    on:click=on_click
                    id="fa-trash-o"
                    class="cursor-pointer text-[24px] ml-2.5 {}".into()
                />
            </td>
        </tr>
    }
}
