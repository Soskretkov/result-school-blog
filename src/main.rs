use leptos::*;
mod components;
mod pages;
mod server;
mod types;
mod utils;
use components::{Footer, Header, PageErrMsg};
use leptos_router::*;
use pages::*;
use server::Session;
use types::GlobContext;
use leptos_use::{storage, utils::JsonCodec};

fn main() {
    leptos::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let (session, set_session, _) = storage::use_local_storage::<Option<Session>, JsonCodec>("user_session");

    view! {
        <Router>
            { provide_context(GlobContext::new(session)) }
            <div class="flex flex-col justify-between bg-white w-[1000px] min-h-screen mx-auto">
                <Header set_session={set_session}/> // btn. "выход" сбрасывает session на None
                <main class="pt-[100px]">
                    <Routes>
                        <Route path="/" view=move || { view!{<div>"Главная страница"</div>} }/>
                        <Route path="/login" view=move || { view!{<Authorization set_session={set_session}/>} }/>
                        <Route path="/register" view=move || { view!{<Registration set_session={set_session}/>} }/>
                        <Route path="/users" view=move || { Users }/>
                        <Route path="/post" view=move || { view!{<div>"Новая статья"</div>} }/>
                        <Route path="/post/:id" view=move || { view!{<Post/>} }/>
                        <Route path="/*" view=move || { view!{ <PageErrMsg>"Запрошенная страница не существует"</PageErrMsg>}}/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}
