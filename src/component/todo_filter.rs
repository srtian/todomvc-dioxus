use crate::{Filter, Todos};
use dioxus::prelude::*;
#[derive(Props)]
pub struct TodoFilterProps<'a> {
    pub use_todo: &'a UseState<Todos>,
    pub use_filter: &'a UseState<Filter>,
}
pub fn todo_filter<'a>(cx: Scope<'a, TodoFilterProps<'a>>) -> Element {
    let use_todo = cx.props.use_todo;
    let use_filter = cx.props.use_filter;

    let filters = use_filter.get();
    let todos = use_todo.get();

    let completed_number = todos.iter().fold(
        0,
        |acc, (_, todo)| if todo.completed { acc } else { acc + 1 },
    );

    let item_text = if completed_number <= 1 {
        "item left"
    } else {
        "items left"
    };

    let is_show_clear_completed = todos.iter().any(|(_, todo)| todo.completed);

    let get_selected_text = |f, filters: &Filter| {
        if filters == &f {
            "selected"
        } else {
            ""
        }
    };
    let all_class = get_selected_text(Filter::All, filters);
    let active_class = get_selected_text(Filter::Active, filters);
    let completed_class = get_selected_text(Filter::Completed, filters);
    rsx!(
        cx,
        (!todos.is_empty()).then(|| rsx! {
            footer { class: "footer",
                span { class: "todo-count",
                    strong { "{completed_number}" },
                    span {" {item_text}" },
                }
                ul { class: "filters",
                    li {
                        a { class: "{all_class}", href: "#/", onclick: move |_| use_filter.set(Filter::All),  "All" },
                    },
                    li {
                    a { class: "{active_class}", href: "#/active", onclick: move |_| use_filter.set(Filter::Active),  "Active" },
                    },
                    li {
                    a { class: "{completed_class}", href: "#/completed", onclick: move |_| use_filter.set(Filter::Completed),  "Completed" },
                    },
                }
                is_show_clear_completed.then(|| rsx! {
                    button {
                        class: "clear-completed",
                        onclick: move |_| {
                            let mut todos = use_todo.make_mut();
                            todos.retain(|_, todo| !todo.completed);
                        },
                        "Clear completed",
                    }
                })
            }
        })
    )
}