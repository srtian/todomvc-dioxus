use crate::Todos;
use dioxus::prelude::*;

#[derive(Props)]
pub struct TodoItemProps<'a> {
    pub id: u32,
    pub use_todo: &'a UseState<Todos>,
}

pub fn todo_item<'a>(cx: Scope<'a, TodoItemProps<'a>>) -> Element {
    let id = cx.props.id;
    let use_todo = cx.props.use_todo;
    let use_is_editing = use_state(&cx, || false);
    let todos = use_todo.get();
    let todo = &todos[&id];
    let completed = if todo.completed { "completed" } else { "" };
    let editing = if *use_is_editing.current() { "editing" } else { "" };
    rsx! { cx, li {
        class: "{completed} {editing}",
        div { class: "view",
            input {
                class: "toggle",
                r#type: "checkbox",
                id: "todo-{todo.id}",
                checked: "{todo.completed}",
                onchange: move |e| {
                    let mut todos = use_todo.make_mut();
                    todos.get_mut(&id).map(|todo| {
                        todo.completed = e.value.parse().unwrap();
                    });
                }
            },
            label {
                onclick: move |_| {
                    use_is_editing.set(true);
                },
                "{todo.content}"
            }
        }
        use_is_editing.then(|| rsx! {
            input {
                class: "edit",
                value: "{todo.content}",
                oninput: move |e| {
                    let mut todos = use_todo.make_mut();
                    todos.get_mut(&id).map(|todo| {
                        todo.content = e.value.to_string();
                    });
                },
                autofocus: "true",
                onkeydown: move |e| {
                    if e.key.as_str() == "Enter" || e.key.as_str() == "Escape" {
                        use_is_editing.set(false);
                    }
                },
            }
        })
    }}
}