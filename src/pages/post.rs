mod components;
use crate::components::{PageErrMsg, H2};
use crate::server::{self};
use components::Comments;
use leptos::*;
use leptos_router::*;

#[component]
pub fn Post() -> impl IntoView {
    view! {
        <Await
            future=move|| {
                let id = use_params_map().with(|params| params.get("id").cloned()).unwrap();
                async move { server::fetch_post(&id).await }
            }
            let: post_wrapped
        >{
            match post_wrapped {
                Ok(post) => {
                    let title = post.title.clone();
                    view! {
                        <div>
                            <img src={&post.image_url} alt={&post.title}/>
                            <H2>{title}</H2>
                            <div>{&post.published_at}</div>
                            <div>{&post.content}</div>
                        </div>
                        <Comments/>
                    }
                    .into_view()
                },
                Err(e) => {
                    let err_msg = e.to_string();
                    view! {<PageErrMsg>{err_msg.clone()}</PageErrMsg>}
                },
            }
        }</Await>
    }
}
