use leptos::*;
use leptos_router::*;
use crate::bff;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Header/>
            <Routes>
                <Route path="/" view=|| view!{<div class="text-red-600">"Главная страница"</div>}/>
                <Route path="/login" view=|| view!{<div>"Авторизация"</div>}/>
                <Route path="/register" view=|| view!{<div>"Регистрация"</div>}/>
                <Route path="/users" view=|| view!{<div>"Пользователи"</div>}/>
                <Route path="/post" view=|| view!{<div>"Новая статья"</div>}/>
                <Route path="/post/:postId" view=|| view!{<div>"Статья"</div>}/>
                <Route path="/post" view=|| view!{<div>"Новая статья"</div>}/>
                <Route path="/*" view=|| view!{<div>"Ошибка"</div>}/>

            </Routes>
            <Footer/>
        </Router>
    }
}

#[component]
pub fn Content() -> impl IntoView {
    view! {
        <h2>Content</h2>
    }
}


#[component]
pub fn Header() -> impl IntoView {
    view! {
        <h1>Header</h1>
    }
}

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <h1>Footer</h1>
    }
}




#[component]
pub fn Load() -> impl IntoView {
    let users = create_resource(|| (), |_| async move {
        bff::db_utils::get_users().await
    });

    view! {
        <div>
            {
                // Отображаем данные или сообщение о загрузке
                match users.get() {
                    Some(data) => view! { <p>{format!("Users: {:?}", data)}</p> },
                    None => view! { <p>{"Loading users..."}</p> }
                }
            }
        </div>
    }
}
