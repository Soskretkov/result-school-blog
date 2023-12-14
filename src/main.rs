mod bff;
mod shared;
use leptos::*;
mod components;
mod pages;
use bff::Server;
use components::{Footer, Header};
use leptos_router::*;
use pages::Authorization;

// #[tokio::main]
fn main() {
    let server = Server::new();
    // let server = leptos::mount_to_body(move || view! {<App server=server/>});
    let server = leptos::mount_to_body(App);
}

// Условности:
// 1) пока Footer не будет перемонтирован, он не обновит погоду и дату
// рендеринг App не вызовет рендеринга/перемонтирования Footer
// В React, когда родительский перерендеривается, это ведет к перерендерингу всей дочерней иерархии.
// 2) Нет валидации авторизации, нет disabled на кнопку
// 3) Нет асинхронных запросов у авторизации
#[component]
pub fn App() -> impl IntoView {
// pub fn App(server: Server) -> impl IntoView {
    view! {
        <Router>
            <div class="flex flex-col justify-between bg-white w-[1000px] min-h-screen mx-auto">
                <Header/>
                <main class="mt-[120px]">
                    <Routes>
                        <Route path="/" view=|| view!{<div>"Главная страница"</div>}/>
                        // <Route path="/login" view= || view!{<Authorization server={&mut server}/>}/>
                        <Route path="/register" view=|| view!{<div>"Регистрация"</div>}/>
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

// #[component]
// pub fn Load() -> impl IntoView {
//     let users = create_resource(|| (), |_| async move { bff::db_utils::get_users().await });

//     view! {
//         <div>
//             {
//                 // Отображаем данные или сообщение о загрузке
//                 match users.get() {
//                     Some(data) => view! { <p>{format!("Users: {:?}", data)}</p> },
//                     None => view! { <p>{"Loading users..."}</p> }
//                 }
//             }
//         </div>
//     }
// }
