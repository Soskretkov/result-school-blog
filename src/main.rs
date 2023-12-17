mod bff;
mod entities;
use leptos::*;
mod components;
mod pages;
use bff::Server;
use components::{Footer, Header};
use entities::User;
use leptos_router::*;
use pages::authentication::{Authorization, Registration};

// #[tokio::main]
fn main() {
    leptos::mount_to_body(App);
}

// Условности:
// 1) пока Footer не будет перемонтирован, он не обновит погоду и дату
// рендеринг App не вызовет рендеринга/перемонтирования Footer
// В React, когда родительский перерендеривается, это ведет к перерендерингу всей дочерней иерархии.
// 2) Нет валидации авторизации, нет disabled на кнопку
// 3) Нет асинхронных запросов у авторизации
// 4) Добавить перенаправление на главную если лигиненый попал на страницы аутентификации
#[component]
pub fn App() -> impl IntoView {
    let (_, set_server) = create_signal(Server::new());
    provide_context(set_server);

    let rw_user = create_rw_signal::<Option<User>>(None);
    let user = rw_user.read_only();

    view! {
        <Router>
            <div class="flex flex-col justify-between bg-white w-[1000px] min-h-screen mx-auto">
                <Header rw_user={rw_user}/>
                <main class="mt-[120px]">
                    <Routes>
                        <Route path="/" view=|| view!{<div>"Главная страница"</div>}/>
                        <Route path="/login" view=move || view!{<Authorization rw_user={rw_user}/>}/>
                        <Route path="/register" view=move || view!{<Registration rw_user={rw_user}/>}/>
                        <Route path="/users" view=|| view!{<div>"Пользователи"</div>}/>
                        <Route path="/post" view=|| view!{<div>"Новая статья"</div>}/>
                        <Route path="/post/:postId" view=|| view!{<div>"Статья"</div>}/>
                        <Route path="/post" view=|| view!{<div>"Новая статья"</div>}/>
                        <Route path="/*" view=|| view!{<div>"Ошибка"</div>}/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}
