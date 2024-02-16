use crate::server::{Role, RoleType};
use crate::types::GlobContext;
use leptos::*;

#[component]
pub fn RoleSelect(
    selected_role_type: RoleType,
    set_is_changed_role: WriteSignal<bool>,
    #[prop(optional)] class: &'static str,
) -> impl IntoView {
    let glob_ctx = use_context::<GlobContext>().unwrap();
    let roles_vec = glob_ctx.roles.value().get_untracked().unwrap().unwrap();

    let on_change = move |ev: ev::Event| {
        set_is_changed_role.set(event_target_value(&ev) != selected_role_type.as_str())
    };

    // возможна оптимизация чтобы не создавать элемент по числу пользователей
    let option = move |role: Role| {
        view! {
            <option
                value=role.id.as_str()
                selected={selected_role_type == role.id}
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
