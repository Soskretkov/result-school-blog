use crate::server::{Role, RoleName};
use leptos::*;

#[component]
pub fn RoleSelect(user_role_id: RoleName, roles_res: Resource<(), Vec<Role>>) -> impl IntoView {
    let option = move |role: Role| {
        view! {
            <option value=role.id.as_u8() selected={user_role_id == role.id}>
                {role.name}
            </option>
        }
    };

    view! {
        <select>
            <Suspense
                fallback=move || view! {<option>"loading"</option>}
            >{
                move || {
                    roles_res
                    .get()
                    .unwrap_or(Vec::new())
                    .into_iter()
                    .map(|role| option(role))
                    .collect_view()
                }
            }</Suspense>
        </select>
    }
}
