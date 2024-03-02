use crate::components::Icon;
use leptos::*;

pub struct Comment {
    pub id: String,
    pub post_id: String,
    pub user_id: String,
    pub login_snapshot: String,
    pub content: String,
    pub published_at: String,
}

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

            <Comment
                comment=Comment {
                    id: "String".to_string(),
                    post_id: "004".to_string(),
                    user_id: "String".to_string(),
                    login_snapshot: "Петя".to_string(),
                    content: "Статья говно, автора передать инквизиции (смайлик с костром)".to_string(),
                    published_at: "2033-12-09".to_string(),
                }
            />
        </div>
    }
}

#[component]
pub fn Comment(comment: Comment) -> impl IntoView {
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
                        <div class = "">{comment.published_at}</div>
                    </div>
                </div>
                <div class="">{comment.content}</div>
            </div>
            <Icon id="fa-trash-o" class="text-[21px]"/>
        </div>
    }
}
