mod tbody_row;
use crate::server::{self, User};
use leptos::*;
use tbody_row::TbodyRow;

#[component]
pub fn TableBody(users_vec: Vec<User>) -> impl IntoView {
    let (users_signal, set_users_signal) = create_signal(users_vec);

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
                            set_users_signal=set_users_signal
                        />
                    }
                }
            />
        </tbody>
    }
}
