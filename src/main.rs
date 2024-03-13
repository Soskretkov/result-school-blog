use leptos::*;
mod components;
mod pages;
mod server;
mod types;
mod utils;
use components::{Footer, Header, PageErrMsg};
use leptos_router::*;
use leptos_use::{storage, utils::JsonCodec};
use pages::*;
use types::Auth;
use types::GlobContext;

fn main() {
    leptos::mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let (auth, set_auth, _) = storage::use_local_storage::<Option<Auth>, JsonCodec>("user_session");

    view! {
        <Router>
            { provide_context(GlobContext::new(auth, set_auth)) }
            { create_user_updater(auth, set_auth) }
            <div class="flex flex-col justify-between bg-white w-[1000px] min-h-screen mx-auto">
                <Header set_auth={set_auth}/> // btn. "выход" сбрасывает auth на None
                <main class="pt-[100px]">
                    <Routes>
                        <Route path="/" view=move || { view!{<div>"Главная страница"</div>} }/>
                        <Route path="/login" view=move || { view!{<Authorization set_auth={set_auth}/>} }/>
                        <Route path="/register" view=move || { view!{<Registration set_auth={set_auth}/>} }/>
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

fn create_user_updater(
    auth: Signal<Option<Auth>>,
    set_auth: WriteSignal<Option<Auth>>,
) -> impl Fn() {
    let current_path = use_location().pathname;
    let user_action = create_action(move |_: &()| async move {
        if auth.with_untracked(Option::is_some) {
            if let Ok(user) = server::fetch_current_user().await {
                match auth
                    .with_untracked(|wr_auth| wr_auth.as_ref().and_then(|auth| auth.user.clone()))
                {
                    Some(stored_user) if stored_user == user => (),
                    _ => {
                        let sess = auth
                            .with_untracked(|wr_auth| wr_auth.as_ref().map(|a| a.session.clone()))
                            .unwrap();
                        set_auth.set(Some(Auth::new(sess, Some(user))));
                    }
                }
            }
        }
    });

    move || {
        current_path.track();
        user_action.dispatch(());
    }
}
