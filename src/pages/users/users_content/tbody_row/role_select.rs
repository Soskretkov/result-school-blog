use crate::server::{Role, RoleName};
use crate::types::GlobContext;
use leptos::*;

#[component]
pub fn RoleSelect(user_role_id: RoleName) -> impl IntoView {
    let glob_ctx = use_context::<GlobContext>().unwrap();
    let roles_vec = glob_ctx.roles.value().get_untracked().unwrap().unwrap();

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
        {roles_vec
            .into_iter()
            .map(|role| option(role))
            .collect_view()}
        </select>
    }
}
