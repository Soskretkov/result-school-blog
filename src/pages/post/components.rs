use crate::components::Icon;
use crate::server::Comment;
use leptos::*;

#[component]
pub fn Comments() -> impl IntoView {
    let (new_comment, set_new_comment) = create_signal("".to_string());

    // on_new_comment_add(post_id)
    // add_post_comment (вернет комментарий?)
    view! {
        <div class="w-[580px] my-0 mx-auto"> // comments class
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

            // <Comment
            //     comment=PostComment {
            //         id: "String".to_string(),
            //         post_id: "004".to_string(),
            //         user_id: "String".to_string(),
            //         login_snapshot: "Петя".to_string(),
            //         content: "Статья говно, автора передать инквизиции (смайлик с костром)".to_string(),
            //         created_at: "2033-12-09".to_string(),
            //     }
            // />
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
