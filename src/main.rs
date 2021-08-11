use serde::{Deserialize, Serialize};
use serde_json;
use sled::{self, Config, Db, Result};
use std::str;

#[derive(Debug, Serialize, Deserialize)]
struct Todo {
    id: String,
    title: String,
    is_done: bool,
}

struct TodoList {
    db: Db,
    lastest_id: String,
}

impl TodoList {
    fn new(path: String) -> Self {
        TodoList {
            db: sled::open(path.as_str()).unwrap(),
            lastest_id: format!("0"),
        }
    }
    fn add(&mut self, title: String) -> Result<()> {
        let id = format!("{}", self.lastest_id.parse::<usize>().unwrap() + 1);
        self.lastest_id = id.clone();
        let todo = Todo {
            id: id.clone(),
            title,
            is_done: false,
        };
        let r = serde_json::to_string(&todo).unwrap();
        self.db.insert(id.as_bytes(), r.as_bytes())?;
        Ok(())
    }

    fn toggle_done(&mut self, id: String) {
        let todo = self.db.get(id.as_bytes()).unwrap();
        match todo {
            Some(todo) => {
                let s = str::from_utf8(&todo).unwrap();
                let mut todo: Todo = serde_json::from_str(s).unwrap();
                todo.is_done = !todo.is_done;
                self.db
                    .insert(
                        id.as_bytes(),
                        serde_json::to_string(&todo).unwrap().as_bytes(),
                    )
                    .unwrap();
            }
            None => {}
        }
    }

    fn get(&self, id: String) -> Option<Todo> {
        let todo = self.db.get(id.as_bytes()).unwrap();
        match todo {
            Some(todo) => {
                let s = str::from_utf8(&todo).unwrap();
                let todo: Todo = serde_json::from_str(s).unwrap();
                Some(todo)
            }
            None => None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut todo_list = TodoList::new("my_db".to_string());
    //todo_list.add(format!("ok"))?;
    //todo_list.add(format!("thisis good"))?;
    let todo = todo_list.get(format!("1"));
    println!("{:?}", todo);
    let todo = todo_list.get(format!("2"));
    println!("{:?}", todo);
    todo_list.toggle_done(format!("2"));
    let todo = todo_list.get(format!("2"));
    println!("{:?}", todo);
    Ok(())
}
