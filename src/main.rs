extern crate api_project;
extern crate serde;
#[macro_use] extern crate rouille;

use api_project::{Task, TaskManager};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

/// External representation of a single task.
#[derive(Serialize)]
struct GetTaskResponse<'a> {
    text: &'a str,
    done: bool,
}

impl<'a> GetTaskResponse<'a> {
    /// Constructs a new external version of a task given a task's definition.
    fn from(t: &'a Task) -> Self {
        Self { text: &t.text, done: t.done }
    }
}

/// Representation of a request to update zero or more fields of a task.
#[derive(Deserialize)]
struct UpdateTaskRequest {
    text: Option<String>,
    done: Option<bool>,
}

/// Processes REST requests for the task manager API and transforms them into
/// operations against the given backing `task_manager`.
fn route_request(request: &rouille::Request, task_manager: &Mutex<TaskManager>)
                 -> rouille::Response {

    let mut task_manager = task_manager.lock().unwrap();

    router!(request,
        (GET) ["/task"] => {
            let mut response = HashMap::new();
            for (id, task) in task_manager.all().iter() {
                let path = format!("/task/{}", id);
                response.insert(path, GetTaskResponse::from(task));
            }
            rouille::Response::json(&response)
        },

        (POST) ["/task"] => {
            let body: UpdateTaskRequest =
                try_or_400!(rouille::input::json_input(request));
            let id = task_manager.add(body.text);
            rouille::Response::json(&format!("/task/{}", id))
        },

        (GET) ["/task/{id}", id: usize] => {
            match task_manager.get(id) {
                Ok(task) =>
                    rouille::Response::json(&GetTaskResponse::from(task)),
                Err(e) => rouille::Response::json(&e).with_status_code(404),
            }
        },

        (UPDATE) ["/task/{id}", id: usize] => {
            let body: UpdateTaskRequest =
                try_or_400!(rouille::input::json_input(request));
            match task_manager.set(id, body.text, body.done) {
                Ok(()) => rouille::Response::empty_204(),
                Err(e) => rouille::Response::json(&e).with_status_code(404),
            }
        },

        (DELETE) ["/task/{id}", id: usize] => {
            match task_manager.delete(id) {
                Ok(()) => rouille::Response::empty_204(),
                Err(e) => rouille::Response::json(&e).with_status_code(404),
            }
        },

        _ => rouille::Response::empty_404()
    )
}

fn main() {
    let task_manager = Mutex::from(TaskManager::new());
    rouille::start_server(
        "localhost:1234", move |request| route_request(request, &task_manager))
}

//curl -X POST -H "Content-Type: application/json" -d '"HEE REVEILLE TOII ICI C EST LES CITES DE FRANCE NARVALO"' http://localhost:1234/task
//curl -X POST -H "Content-Type: application/json" -d '"HEE LAISSEZ MOI DORMIR ZEBI"' http://localhost:1234/task
//curl -X POST -H "Content-Type: application/json" -d '"BASSEM IL S EST FAIT ENCULER"' http://localhost:1234/task
//curl -X GET http://localhost:1234/task/
//curl -X GET http://localhost:1234/task/1
//curl -X UPDATE -H "Content-Type: application/json" -d '{"done": true}' http://localhost:1234/task/0
//curl -X DELETE http://localhost:1234/task/