mod role_select;
mod save_icon;
use crate::components::Icon;
use crate::server::{self, RoleType, User};
use crate::utils;
use leptos::*;
use role_select::RoleSelect;
use save_icon::SaveIcon;

#[component]
pub fn TbodyRow(user: User) -> impl IntoView {
    let (role_type, set_role_type) = create_signal(user.role_id);
    let (new_role_type, set_new_role_type) = create_signal(user.role_id);

    let on_save = {
        let save_action = create_action(move |arg: &(String, RoleType, WriteSignal<RoleType>)| {
            let user_id = arg.0.clone();
            let user_new_role_type = arg.1;
            let set_role_type = arg.2;

            async move {
                if utils::is_sync_server_client_roles() {
                    if server::update_user_role(&user_id, user_new_role_type)
                        .await
                        .is_ok()
                    {
                        set_role_type.set(user_new_role_type);
                    }
                }
            }
        });

        move |_: ev::MouseEvent| {
            save_action.dispatch((user.id.clone(), new_role_type.get(), set_role_type));
        }
    };

    // удалить в бд, запросить пользователей, обновить ресурс или сигнал
    let on_delete = |_: ev::MouseEvent| unimplemented!();

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
                    <SaveIcon
                        on:click=on_save.clone()
                        is_deactive = role_type.get() == new_role_type.get()
                    />
                }}
            </td>
            <td class="w-auto flex items-center">
                <Icon
                    on:click=on_delete
                    id="fa-trash-o"
                    class="cursor-pointer text-[24px] ml-2.5 {}".into()
                />
            </td>
        </tr>
    }
}
