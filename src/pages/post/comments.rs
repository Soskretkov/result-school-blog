use crate::components::Icon;
use crate::server::{self, Comment};
use leptos::{ev::SubmitEvent, html::Textarea, *};

#[component]
pub fn Comments(post_id: String, comments: Vec<Comment>) -> impl IntoView {
    let (comments_signal, set_comments_signal) = create_signal(comments);
    let comment_node_ref = create_node_ref::<Textarea>();

    let on_new_comment_add = {
        let add_action = create_action(move |&()| {
            let post_id_clon = post_id.clone();
            let new_content = comment_node_ref.get().unwrap().value();

            async move {
                server::add_comment(post_id_clon, new_content)
                    .await
                    .map(|cmnt| set_comments_signal.update(|vec| vec.push(cmnt)))
            }
        });

        move |ev: SubmitEvent| {
            ev.prevent_default();
            add_action.dispatch(());
        }
    };

    view! {
        <div class="w-[580px] my-0 mx-auto"> // comments class
            // форма добавления нового комментария
            <form on:submit = on_new_comment_add class="flex w-full mt-[20px] items-start"> // new-comment class
                <textarea
                    name="comment"
                    value={""}
                    placeholder="Комментарий..."
                    class="text-[18px] pl-[2px] w-full h-[120px] border border-black rounded-sm outline-none"
                    node_ref = comment_node_ref
                />
                <button
                    type="submit"
                >
                    <Icon
                        id="fa-paper-plane-o"
                        class="text-[18px] ml-[10px] cursor-pointer"
                    />
                </button>
            </form>


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
                        {comment.user_name_snapshot}
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
