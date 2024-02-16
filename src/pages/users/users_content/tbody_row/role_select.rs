use crate::server::{Role, RoleType};
use crate::types::GlobContext;
use leptos::*;

#[component]
pub fn RoleSelect(
    initial_role_type: RoleType,
    set_new_role_type: WriteSignal<RoleType>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let glob_ctx = use_context::<GlobContext>().unwrap();
    let roles_vec = glob_ctx.roles.value().get_untracked().unwrap().unwrap();

    let on_change = move |ev: ev::Event| {
        let new_u8_role = event_target_value(&ev).parse::<u8>().unwrap();
        set_new_role_type.set(RoleType::from_u8(new_u8_role).unwrap());
    };

    // возможна оптимизация чтобы не создавать элемент по числу пользователей
    let option = move |role: Role| {
        view! {
            <option
                value=role.id.as_u8()
                selected={initial_role_type == role.id}
            >
                {role.name}
            </option>
        }
    };

    // стилизация
    let preset_classes = "text-[16px] ml-[5px]";
    let class_list = format!("{preset_classes} {class}");

    view! {
        <select
            class=class_list
            on:change = on_change
        >
            {roles_vec
                .into_iter()
                .map(|role| option(role))
                .collect_view()
            }
        </select>
    }
}
