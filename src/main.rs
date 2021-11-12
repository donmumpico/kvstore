use std::{collections::HashMap};

fn main() {
    let mut args = std::env::args().skip(1);
    let key = args.next().expect("key expected");
    let value = args.next().expect("value expected");
    println!("The key is '{}' and the value is '{}'", key, value);
    
    let database = Database::new().expect("Database::new() crashed");
    database.insert(key, value);
}

struct Database {
    map: HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut map = HashMap::new();
        /*
        let contents = match std::fs::read_to_string("kv.db") {
           Ok(c) => c,
           Err(error) => {
               return Err(error);
           }
        };
        */
        let contents = std::fs::read_to_string("kv.db")?;

        for line in contents.lines() {
            let (key, value) = line.split_once('\t').expect("corrupt database");
            map.insert(key.to_owned(), value.to_owned());
        }

        Ok(Database {map: map})
    }

    fn insert(mut self, key: String, value: String) {
        self.map.insert(key, value);
    }
}
