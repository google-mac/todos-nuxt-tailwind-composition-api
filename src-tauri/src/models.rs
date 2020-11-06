use chrono::{DateTime, Utc};
use jfs::{Config, Store};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct AppData;

impl AppData {
    fn get_store() -> Store {
        let app_dir_path = tauri::api::path::app_dir().expect("Couldn't determine app directory");

        if !app_dir_path.exists() {
            std::fs::create_dir(&app_dir_path).expect("Couldn't create app directory");
        }

        let data_path = &app_dir_path.join("data");
        let path = data_path.to_str().expect("Couldn't build app data file");

        dbg!(&path);

        let mut cfg = Config::default();
        cfg.single = true; // store in single file
        if cfg!(debug_assertions) {
            cfg.pretty = true;
            cfg.indent = 4;
        }

        Store::new_with_cfg(&path, cfg).expect("Couldn't initialize store")
    }

    pub fn get_todos() -> Vec<Todo> {
        let mut todos = Self::get_store()
            .all()
            .expect("Can't get all from store")
            .values()
            .cloned()
            .collect::<Vec<Todo>>();

        todos.sort_by(|a, b| a.created_at.cmp(&b.created_at));

        todos
    }

    pub fn create_todo(todo: &Todo) {
        Self::get_store()
            .save_with_id(todo, &todo.id)
            .expect("Couldn't add todo");
    }

    pub fn update_todo(todo: &Todo) {
        Self::get_store()
            .save_with_id(todo, &todo.id)
            .expect("Couldn't update todo");
    }

    pub fn remove_todo(id: String) {
        Self::get_store().delete(&id).expect("Couldn't delete todo")
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Todo {
    id: String,
    created_at: DateTime<Utc>,
    title: String,
    completed: bool,
}

impl Todo {
    pub fn new_with_title(title: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let created_at = Utc::now();

        Todo {
            id,
            created_at,
            title,
            completed: false,
        }
    }
}
