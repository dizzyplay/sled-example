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

    fn get(&self, id: String) -> Option<Todo> {
        let todo = self.db.get(id.as_bytes()).unwrap();
        match todo {
            Some(todo) => {
                let s = str::from_utf8(&todo).unwrap();
                let todo: Todo = serde_json::from_str(s).unwrap();
                println!("{}", s);
                Some(todo)
            }
            None => None,
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut todo_list = TodoList::new("new_todos".to_string());
    todo_list.add(format!("ok"))?;
    todo_list.add(format!("thisis good"))?;
    let todo = todo_list.get(format!("1"));
    println!("{:?}", todo);
    let todo = todo_list.get(format!("2"));
    println!("{:?}", todo);
    Ok(())
}

async fn n() -> Result<()> {
    let config = Config::new().temporary(true);
    fn m(_k: &[u8], old: Option<&[u8]>, new: &[u8]) -> Option<Vec<u8>> {
        println!("e");
        Some(new.to_vec())
    }
    let db = config.open()?;
    db.set_merge_operator(m);
    let k = b"1".to_vec();
    db.merge(k.clone(), br#"{"a":1}"#)?;
    let r = db.get(k)?;
    println!("{}", std::str::from_utf8(&r.unwrap()).unwrap());

    my(db.clone()).await?;

    Ok(())
}

async fn my(db: sled::Db) -> Result<()> {
    db.merge(b"ok", b"this is async fn in")?;
    let r = db.get(b"1").unwrap();
    println!("{}", std::str::from_utf8(&r.unwrap()).unwrap());
    Ok(())
}
