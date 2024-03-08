use crate::components::Icon;
use crate::server::{self, Comment};
use leptos::*;

#[component]
pub fn Comments(post_id: String, comments: Vec<Comment>) -> impl IntoView {
    let (comments_signal, set_comments_signal) = create_signal(comments);

    let add_action = create_action(move |new_content: &String| {
        let new_content_clon = new_content.clone();
        let post_id_clon = post_id.clone();

        async move {
            server::add_comment(post_id_clon, new_content_clon)
                .await
                .map(|cmnt_type| set_comments_signal.update(|vec| vec.push(cmnt_type)))
        }
    });

    // on_new_comment_add(post_id)
    view! {
        <div class="w-[580px] my-0 mx-auto"> // comments class
            // форма добавления нового комментария
            <div class="flex w-full mt-[20px] items-start"> // new-comment class
                <textarea
                    name="comment"
                    value={""}
                    placeholder="Комментарий..."
                    class="text-[18px] pl-[2px] w-full h-[120px] border border-black rounded-sm outline-none"
                    on:change=move|_| {}
                />
                <Icon
                    id="fa-paper-plane-o"
                    class="text-[18px] ml-[10px] cursor-pointer"
                />
            </div>

            <For // все существующие комментарии
                each=move || comments_signal.get()
                key=|cmnt| cmnt.id.clone()
                children=move |cmnt| {
                    view! {
                        <Comment comment=cmnt />
                    }
                }
            />
        </div>
    }
}

#[component]
pub fn Comment(comment: Comment) -> impl IntoView {
    let created_at = comment.created_at.format("%Y-%m-%d %H:%M").to_string();

    view! {
        <div class="flex">
            <div class="">
                <div class="flex justify-between"> // information-panel class
                    <div class="flex"> // author class
                        <Icon id="fa-user-circle-o" class="text-[18px] mr-[7px]"/>
                        {comment.login_snapshot}
                    </div>
                    <div class="flex">
                        <Icon id="fa-calendar-o" class="top-[-1px] text-[18px] mr-[7px]"/>
                        <div class = "">{created_at}</div>
                    </div>
                </div>
                <div class="">{comment.content}</div>
            </div>
            <Icon id="fa-trash-o" class="text-[21px]"/>
        </div>
    }
}
