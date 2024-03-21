use crate::components::Icon;
use crate::server::{self, Comment};
use crate::types::ModalWindow;
use leptos::*;

#[component]
pub fn Comment(comment: Comment, set_comments_signal: WriteSignal<Vec<Comment>>) -> impl IntoView {
    let comment_id = store_value(comment.id);
    
    let on_delete = {
        let delete_action = create_action(move |_: &()| async move {
            if server::remove_comment(&comment_id.get_value())
                .await
                .is_ok()
            {
                set_comments_signal.update(|vec| {
                    vec.retain(|cmnt| comment_id.with_value(|comment_id| &cmnt.id != comment_id))
                })
            }
        });

        move |_: ev::MouseEvent| {
            ModalWindow::set(
                "Удалить комментарий?".to_string(),
                move || delete_action.dispatch(()),
            )
        }
    };

    let created_at = comment.created_at.format("%Y-%m-%d %H:%M").to_string();

    view! {
        <div class="flex mt-[10px]"> // ComentContainer
            <div class="w-[550px] py-[5px] px-[10px] border border-black rounded-sm"> // comment class
                <div class="flex justify-between"> // information-panel class
                    <div class="flex"> // author class
                        <Icon id="fa-user-circle-o" class="text-[18px] mr-[10px]"/>
                        {comment.user_name_snapshot}
                    </div>
                    <div class="flex"> // published_at class
                        <Icon
                            id="fa-calendar-o"
                            class="text-[18px] mr-[10px] top-[-1px]"
                        />
                        {created_at}
                    </div>
                </div>
                <div class="">{comment.content}</div> // comment-text class
            </div>
            <Icon
                on:click=on_delete
                id="fa-trash-o"
                class="text-[21px] ml-[10px] cursor-pointer"
            />
        </div>
    }
}
