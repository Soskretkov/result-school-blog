use leptos::*;
mod components;
mod pages;
mod server;
mod types;
mod utils;
use components::{Footer, Header};
use leptos_router::*;
use pages::{Authorization, Registration, Users};
use server::Session;
use types::{GlobContext, UserInfo};

fn main() {
    leptos::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let (session, set_session) = create_signal::<Option<Session>>(None);
    view! {
        <Router>
            {provide_context(GlobContext {
                session,                           // Authorization, Registration, struct UserInfo
                user_info: UserInfo::new(session), // Header, Users
                roles: create_action(move |_: &()| async move { server::fetch_all_roles().await }),
            });}
            <div class="flex flex-col justify-between bg-white w-[1000px] min-h-screen mx-auto">
                <Header set_session={set_session}/> // btn. "выход" сбрасывает rw_session на None
                <main class="pt-[120px]">
                    <Routes>
                        <Route path="/" view=move || {
                            view!{<div>"Главная страница"</div>}
                        }/>
                        <Route path="/login" view=move || {
                            view!{<Authorization set_session={set_session}/>}
                        }/>
                        <Route path="/register" view=move || {
                            view!{<Registration set_session={set_session}/>}
                        }/>
                        // <Route path="/users" view=move || {
                        //     view! {
                        //         <Show
                        //             when=move|| session.with(Option::is_some)
                        //             fallback=|| view!{<div>"Ошибка"</div>}
                        //         >
                        //             <Outlet/>
                        //         </Show>
                        //     }
                        // }>
                        //     <Route path="" view=|| view!{<Users/>}/>
                        // </Route>

                        <Route path="/users" view=move || {
                            Users
                        }/>
                        <Route path="/post" view=move || {
                            view!{<div>"Статьи"</div>}
                        }/>
                        <Route path="/post/:postId" view=move || {
                            view!{<div>"Статья"</div>}
                        }/>
                        <Route path="/*" view=move || {
                            view!{<div>"Ошибка"</div>}
                        }/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}

// fn update_location_on_navigation(set_location: WriteSignal<String>) -> impl Fn() {
//     let current_path = use_location().pathname;
//     let (is_app_start, set_is_app_start) = create_signal(true);

//     move || {
//         if !is_app_start.get() {
//             logging::log!("Переход на страницу: {}", current_path.get());
//             set_location.set(current_path.get());
//         } else {
//             current_path.track();
//             set_is_app_start.set(false);
//         }
//     }
// }
