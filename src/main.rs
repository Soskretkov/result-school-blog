use bff;
use leptos::*;
mod components;
mod pages;
mod types;
use bff::server::Session;
use components::{Footer, Header};
use leptos_router::*;
use pages::{Authorization, Registration, Test, Users};
use types::{GlobContext, UserInfo};

fn main() {
    leptos::mount_to_body(App);
}

// Условности:
// 1) Нет валидации авторизации, нет disabled на кнопку
// 2) Нет асинхронных запросов у авторизации, регистрации
// 3) Страница Users: вход только с правами в принципе, логика похожа на login.rs (Header)
#[component]
pub fn App() -> impl IntoView {
    let rw_session = create_rw_signal::<Option<Session>>(None);
    let (location, set_location) = create_signal("".to_string());

    provide_context(GlobContext {
        location,
        session: rw_session.read_only(),
        user_info: UserInfo::new(rw_session),
    });

    view! {
        <Router>
            {
                let current_path = use_location().pathname;
                move || {
                    logging::log!("Переход на страницу: {}", current_path.get());
                    set_location.set(current_path.get());
                }
            }
            <div class="flex flex-col justify-between bg-white w-[1000px] min-h-screen mx-auto">
                <Header rw_session={rw_session}/> // btn. "выход" сбросывает rw_session на None
                <main class="mt-[120px]">
                    <Routes>
                        <Route path="/" view=|| view!{<div>"Главная страница"</div>}/>
                        <Route path="/login" view=move || view!{<Authorization rw_session={rw_session}/>}/>
                        <Route path="/register" view=move || view!{<Registration rw_session={rw_session}/>}/>
                        <Route path="/users" view=|| view!{<Users/>}/>
                        <Route path="/post" view=move || view!{<Test/>}/>
                        <Route path="/post/:postId" view=|| view!{<div>"Статья"</div>}/>
                        <Route path="/*" view=|| view!{<div>"Ошибка"</div>}/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}
