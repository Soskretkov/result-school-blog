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
use types::GlobContext;

fn main() {
    leptos::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let (session, set_session) = create_signal::<Option<Session>>(None);

    view! {
        <Router>
            { provide_context(GlobContext::new(session)) }
            <div class="flex flex-col justify-between bg-white w-[1000px] min-h-screen mx-auto">
                <Header set_session={set_session}/> // btn. "выход" сбрасывает session на None
                <main class="pt-[120px]">
                    <Routes>
                        <Route path="/" view=move || { view!{<div>"Главная страница"</div>} }/>
                        <Route path="/login" view=move || { view!{<Authorization set_session={set_session}/>} }/>
                        <Route path="/register" view=move || { view!{<Registration set_session={set_session}/>} }/>
                        <Route path="/users" view=move || { Users }/>
                        <Route path="/post" view=move || { view!{<div>"Статьи"</div>} }/>
                        <Route path="/post/:postId" view=move || { view!{<div>"Статья"</div>} }/>
                        <Route path="/*" view=move || { view!{<div>"Ошибка"</div>} }/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}
