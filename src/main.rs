use std::{collections::HashMap, env::home_dir};
use colored::*;
use serde::{Serialize, Deserialize};

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let mut todo = Todo::new().expect("Initialisation of db failed");
    
    if action == "add" {
        let item = std::env::args().nth(2).expect("Please specify an item");
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("todo saved"),
            Err(why) => println!("An error occurred: {}", why),
        }
    }
    else if action == "complete" {
        let item = std::env::args().nth(2).expect("Please specify an item");
        match todo.complete(&item){
            None => println!("'{}' is not present in list", item),
            Some(_) => match todo.save() {
                Ok(_) => println!("todo saved"),
                Err(e) => println!("Error occurred: {}", e),
            },
        }
    }
    else if action == "list" {
        todo.print();
    }
    else if action == "--help"{
        println!(r##"todo_cli [COMMAND] "[ITEM]""##);
        println!();
        println!("Commands:");
        println!(r##"todo_cli add "[ITEM]"         adds item to todo list"##);
        println!(r##"todo_cli complete "[ITEM]"    marks item as completed"##);
        println!(r##"todo_cli list "[ITEM]"        prints todo items and status"##);
    }
}

#[derive(Serialize, Deserialize)]
struct Todo {
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(home_dir().unwrap().join(".db.json"))?;
        
        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo { map }),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occurred: {}", e),
        }
    }
    
    fn insert(&mut self, key: String) {
        self.map.insert(key, true);
    }
    
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(home_dir().unwrap().join(".db.json"))?;
        serde_json::to_writer_pretty(f, &self.map)?;
        Ok(())
    }
    
    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut(key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
    
    fn print(&self) {
        println!("{}", "Todo List:".bold().underline());
        let max_item_length = self.map.keys().map(|k| k.len()).max().unwrap_or(0);
        for (item, status) in &self.map {
            let status_str = if *status { 
                "Ongoing".bright_yellow() 
            } else { 
                "Completed".bright_green() 
            };
            println!("{:<width$} {:>10}", item, status_str, width = max_item_length);
        }
    }
}
