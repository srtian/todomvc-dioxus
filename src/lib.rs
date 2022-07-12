mod component;
use dioxus::prelude::*;

use component::{todo_filter, todo_input, todo_item};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct TodoItem {
    pub id: u32,
    pub content: String,
    pub completed: bool,
}
pub type Todos = HashMap<u32, TodoItem>;

#[derive(Debug, Clone, PartialEq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Default for Filter {
    fn default() -> Self {
        Filter::All
    }
}

pub fn app(cx: Scope) -> Element {
    let use_todo = use_state(&cx, Todos::default);
    let use_filter = use_state(&cx, Filter::default);

    let todo_list = use_todo
        .iter()
        .filter(|(_, todo)| match use_filter.get() {
            Filter::All => true,
            Filter::Active => !todo.completed,
            Filter::Completed => todo.completed,
        })
        .map(|(id, _)| *id)
        .collect::<Vec<_>>();

    cx.render(rsx! {
        section { class: "todoapp",
            style { [include_str!("index.css")] },
            div {
                rsx!(todo_input(use_todo: use_todo))
                ul { class: "todo-list",
                    todo_list.iter().map(|id| {
                        rsx!(todo_item(id: *id, use_todo: use_todo))
                    })
                }
                rsx!(todo_filter(use_todo: use_todo, use_filter: use_filter))
            }
        }
    })
}