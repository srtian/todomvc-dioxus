use std::sync::atomic::{AtomicU32, Ordering};
use dioxus::prelude::*;
use crate::{TodoItem, Todos};

static NEXT_TODO_ID: AtomicU32 = AtomicU32::new(1);

#[derive(Props)]
pub struct TodoInputProps<'a> {
    pub use_todo: &'a UseState<Todos>,
}
pub fn todo_input<'a>(cx: Scope<'a, TodoInputProps<'a>>) -> Element {
    let use_todo = cx.props.use_todo;
    let use_draft = use_state(&cx, || String::from(""));

    rsx! {cx,
        header { class: "header",
            h1 { "todos" },
            input {
                class: "new-todo",
                placeholder: "What needs to be done?",
                value: "{use_draft}",
                oninput: move |e| {
                    let value = e.value.to_string();
                    use_draft.set(value);
                },
                onkeydown: move |e| {
                    if e.key == "Enter" && !use_draft.is_empty() {
                        let id = NEXT_TODO_ID.fetch_add(1, Ordering::Relaxed);
                        use_todo.make_mut().insert(id, TodoItem {
                            id,
                            content: use_draft.to_string(),
                            completed: false,
                        });
                        use_draft.set(String::from(""));
                    }
                }
            }
        }
    }
}