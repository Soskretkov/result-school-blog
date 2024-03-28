mod comments;
mod post_content;
use crate::components::{Icon, Input, PageErrMsg};
use crate::server::{self, PostWC};
use comments::Comments;
use leptos::*;
use leptos_router::*;
use post_content::PostContent;

#[component(transparent)]
pub fn Post() -> impl IntoView {
    let post_wc_stored_value: StoredValue<Option<PostWC>> = store_value(None);
    let get_post_id = || {
        use_params_map()
            .with_untracked(|params| params.get("id").cloned())
            .unwrap()
    };

    view! {
        <Route path=":id" view=move || {
            view!{
                <div class="px-20 my-10">
                    <Await
                        future=move|| {
                            // не будет параметров если вызывать get_post_id() внутри async
                            let post_id = get_post_id();
                            let resp = async move {
                                server::fetch_post_wc(&post_id).await
                            };
                            resp
                        }
                        let: post_wc_wrapped
                    >{
                        match post_wc_wrapped {
                            Ok(post_wc) => {
                                post_wc_stored_value.set_value(Some(post_wc.clone()));
                                view! {<Outlet/>}
                            },
                            Err(e) => {
                                let err_msg = e.to_string();
                                view! {<PageErrMsg>{err_msg.clone()}</PageErrMsg>}
                            },
                        }
                    }</Await>
                </div>
            }
        }>
            <Route path="" view=move || {
                post_wc_stored_value.get_value().map(|post| view!{
                    <PostContent post=post.clone()/>
                    <Comments post_id=get_post_id() comments={post.comments}/>
                })
            }/>
            <Route path="edit" view=move || {
                post_wc_stored_value.get_value().map(|post_wc| view!{
                    <PostForm post=post_wc.clone()/>
                })
            }/>
        </Route>
    }
}

#[component]
pub fn PostForm(post: PostWC) -> impl IntoView {
    let created_at = post.created_at.format("%Y-%m-%d").to_string();
    view! {
        <div>
            <Input default={post.image_url}/>
            <Input default={post.title}/>
            <div class="flex justify-between mt-[-20px] mb-5"> // верстка: special-panel у автора
                <div class="flex"> // верстка: published_at у автора
                    <Icon id="fa-calendar-o" class="relative top-[-1px] text-[18px] mr-[7px]"/>
                    <div class = "text-[18px]">{created_at}</div>
                </div>
                <div class="flex"> // верстка: buttons у автора
                    <Icon id="fa-pencil-square-o" class="text-[21px] mr-[10px]"/>
                    <Icon id="fa-trash-o" class="text-[21px]"/>
                </div>
            </div>
            <div class="text-[18px]">{&post.content}</div>
        </div>
    }
}
