mod tbody_row;
use crate::server::{self, User};
use leptos::*;
use tbody_row::TbodyRow;

#[component]
pub fn TableBody(users_vec: Vec<User>) -> impl IntoView {
    let (users_signal, set_users_signal) = create_signal(users_vec);

    let delete_action = create_action(move |user_id: &String| {
        let current_user_id = user_id.clone();

        async move {
            if server::remove_user(&current_user_id).await.is_ok() {
                set_users_signal.update(|vec| vec.retain(|user| current_user_id != user.id))
            }
        }
    });

    view! {
        <tbody class="[&>*>*:not(:last-child)]:border-y [&>*>*:first-child]:border-l [&>*>*:last-child]:border-l [&>*>*]:border-black">
            <For
                each=move || users_signal.get()
                key=|user| user.id.clone()
                children=move |user| {
                    let current_user_id = user.id.clone();
                    view! {
                        <TbodyRow
                            user={user}
                            on_delete=move |_: ev::MouseEvent| {
                                delete_action.dispatch(current_user_id.clone());
                            }
                        />
                    }
                }
            />
        </tbody>
    }
}
