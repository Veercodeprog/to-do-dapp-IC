use candid::CandidType;
use serde::{Deserialize, Serialize};

use ic_cdk::storage;
use std::cell::RefCell;
use std::collections::BTreeMap;

#[derive(Default)]
struct Todos {
    todos: BTreeMap<u32, Todo>,
}

#[derive(Clone, CandidType, Deserialize, Serialize)]
struct Todo {
    id: u32,
    text: String,
    completed: bool,
}

impl Todos {
    fn add_todo(&mut self, text: String) -> Todo {
        let id = self.todos.keys().max().unwrap_or(&0) + 1;
        let todo = Todo {
            id,
            text,
            completed: false,
        };
        self.todos.insert(id, todo.clone());
        todo
    }

    fn remove_todo_by_id(&mut self, id: u32) -> Option<Todo> {
        self.todos.remove(&id)
    }

    fn get_todo_by_id(&self, id: u32) -> Option<Todo> {
        self.todos.get(&id).cloned()
    }

    fn get_todos_paginates(&self, offset: u32, limit: u32) -> Vec<Todo> {
        self.todos
            .values()
            .skip(offset as usize)
            .take(limit as usize)
            .cloned()
            .collect()
    }

    fn update_todo_by_id(
        &mut self,
        id: u32,
        text: Option<String>,
        completed: Option<bool>,
    ) -> Option<Todo> {
        self.todos.get_mut(&id).map(|todo| {
            if let Some(text) = text {
                todo.text = text;
            }
            if let Some(completed) = completed {
                todo.completed = completed;
            }
            todo.clone()
        })
    }
}

thread_local! {
    static STATE: RefCell<Todos> = RefCell::new(Todos::default());
}

#[ic_cdk::update]
fn add(text: String) -> Todo {
    STATE.with(|state| state.borrow_mut().add_todo(text))
}

#[ic_cdk::update]
fn remove(id: u32) -> Option<Todo> {
    STATE.with(|state| state.borrow_mut().remove_todo_by_id(id))
}

#[ic_cdk::query]
fn get(id: u32) -> Option<Todo> {
    STATE.with(|state| state.borrow().get_todo_by_id(id))
}

#[ic_cdk::query]
fn paginate(offset: u32, limit: u32) -> Vec<Todo> {
    STATE.with(|state| state.borrow().get_todos_paginates(offset, limit))
}

#[ic_cdk::update]
fn update(id: u32, text: Option<String>, completed: Option<bool>) -> Option<Todo> {
    STATE.with(|state| state.borrow_mut().update_todo_by_id(id, text, completed))
}
