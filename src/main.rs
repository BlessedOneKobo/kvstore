fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().expect("failed to provide command");

    if command != "get" && command != "set" && command != "get_all" {
        panic!("invalid command");
    }

    let mut database = Database::new().expect("failed to init database");
    let key;
    let value;

    if command == "set" {
        key = args.next().expect("failed to provide key");
        value = args
            .next()
            .expect("failed to provide value for set command");
        database.insert(key, value);
        database.save();
        // database.insert("hello".to_owned(), "world".to_owned());
    } else if command == "get" {
        key = args.next().expect("failed to provide key");
        database.get(key);
    } else if command == "get_all" {
        database.get_all();
    }
}

#[derive(Debug)]
struct Database {
    entries: std::collections::HashMap<String, String>,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut entries: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();

        if std::path::Path::new("kv.db").is_file() == false {
            std::fs::File::create_new("kv.db").expect("failed to create db file");
        }

        let file_contents = std::fs::read_to_string("kv.db")?;
        for line in file_contents.lines() {
            let (key, value) = line.split_once(',').expect("failed to parse db file");
            entries.insert(key.to_owned(), value.to_owned());
        }
        return Ok(Database { entries });
    }

    fn insert(&mut self, key: String, value: String) {
        self.entries.insert(key, value);
    }

    fn get(&self, key: String) {
        match self.entries.get(&key) {
            Some(value) => println!("{}", value),
            None => println!("no value for '{}'", key),
        }
    }

    fn get_all(&self) {
        for (key, value) in &self.entries {
            println!("{},{}", key, value);
        }
    }

    fn save(self) {
        let mut file_contents = String::new();
        for (key, value) in self.entries {
            let line = format!("{},{}", key, value);
            file_contents.push_str(&line);
        }
        std::fs::write("kv.db", file_contents).expect("failed to save contents to db");
    }
}
