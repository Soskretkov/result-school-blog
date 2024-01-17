use crate::server::{Role, RoleName};
use leptos::*;

#[component]
pub fn RoleSelect(user_role_id: RoleName, roles_res: Vec<Role>) -> impl IntoView {
    // возможна оптимизация чтобы не создавать элемент по числу пользователей
    let option = move |role: Role| {
        view! {
            <option value=role.id.as_u8() selected={user_role_id == role.id}>
                {role.name}
            </option>
        }
    };

    view! {
        <select>
        {roles_res
            .into_iter()
            .map(|role| option(role))
            .collect_view()}
        </select>
    }
}
