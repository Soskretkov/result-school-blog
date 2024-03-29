use crate::components::{Icon, H2};
use crate::server::Post;
use leptos::*;

#[component]
pub fn PostContent(post: Post) -> impl IntoView {
    let created_at = post.created_at.format("%Y-%m-%d").to_string();
    
    view! {
        <div>
            <img class="float-left mr-5 mb-2.5" src={&post.image_url} alt={&post.title}/>
            <H2>{&post.title}</H2>
            // верстка: special-panel у автора
            <div class="flex justify-between mt-[-20px] mb-5">
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
