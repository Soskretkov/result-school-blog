use crate::components::Icon;
mod comment;
use crate::server::{self, Comment as CommentType};
use comment::Comment;
use leptos::{ev::SubmitEvent, html::Textarea, *};

#[component]
pub fn Comments(post_id: String) -> impl IntoView {
    let post_id = store_value(post_id);
    view! {
        <Await
            future=move|| {
                let resp = async move {
                    server::fetch_post_comments(&post_id.get_value()).await
                };
                resp
            }
            let: comments_wrapped
        >{
            comments_wrapped.as_ref().ok().map(|comments_vec| view!{
                <CommentsContent post_id=post_id.get_value() comments=comments_vec.clone()/>
            })
        }</Await>
    }
}

#[component]
fn CommentsContent(post_id: String, comments: Vec<CommentType>) -> impl IntoView {
    let (comments_signal, set_comments_signal) = create_signal(comments);
    let comment_node_ref = create_node_ref::<Textarea>();

    let on_new_comment_add = {
        let add_action = create_action(move |comment_html_node: &HtmlElement<Textarea>| {
            let post_id_clone = post_id.clone();
            let comment_html_node_clone = comment_html_node.clone();
            let comment_text = comment_html_node_clone.value();

            async move {
                server::add_comment(post_id_clone, comment_text)
                    .await
                    .map(|cmnt| {
                        set_comments_signal.update(|vec| vec.push(cmnt));
                        comment_html_node_clone.set_value("");
                    })
            }
        });

        move |ev: SubmitEvent| {
            ev.prevent_default();
            let comment_html_node = comment_node_ref.get().unwrap();
            let comment_text = comment_html_node.value();

            if !comment_text.is_empty() {
                add_action.dispatch(comment_html_node);
            }
        }
    };

    view! {
        <div class="w-[580px] mt-0 mx-auto"> // CommentsConainer
            // форма добавления нового комментария
            <form on:submit = on_new_comment_add class="flex w-full mt-[20px] items-start"> // new-comment
                <textarea
                    name="comment"
                    value={""}
                    placeholder="Комментарий . . . "
                    class="w-[550px] h-[120px] text-[18px] pl-[2px]  border border-gray-500 rounded-sm outline-none"
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
                        <Comment
                            comment=cmnt.clone()
                            set_comments_signal=set_comments_signal
                        />
                    }
                }
            />
        </div>
    }
}
