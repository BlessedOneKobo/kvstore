fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().expect("please enter a command");
    let mut database = Database::new().expect("failed to initialize database");

    if command == "get_all" {
        for (key, value) in &database.values {
            println!("{},{}", key, value);
        }
        return;
    }

    let key = args.next().expect("missing key");

    if command == "set" {
        let value = args.next().expect("missing value");
        database.insert(key.to_owned(), value.to_owned());
        database.save();
        println!("wrote ({},{})", key, value);
        return;
    }

    if command == "get" {
        match database.get(key.to_owned()) {
            Some(value) => println!("{}", value),
            None => println!("'{}' does not exist", key),
        }
        return;
    }

    panic!("Invalid command '{}'", command);
}

struct Database {
    values: std::collections::HashMap<String, String>,
    did_flush: bool,
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut values: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let file_contents = std::fs::read_to_string("kv.db")?;
        for line in file_contents.lines() {
            let (key, value) = line.split_once(',').expect("failed to parse db");
            values.insert(key.to_owned(), value.to_owned());
        }

        return Result::Ok(Database {
            values,
            did_flush: false,
        });
    }

    fn get(&self, key: String) -> Option<&String> {
        return self.values.get(&key);
    }

    fn insert(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }

    fn save(&mut self) {
        if self.did_flush {
            panic!("cannot save flushed database");
        }

        flush(self);
        self.did_flush = true;
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        flush(self);
    }
}

fn flush(database: &mut Database) {
    let mut file_contents = String::new();
    for (key, value) in &database.values {
        let line = format!("{},{}\n", key, value);
        file_contents.push_str(&line);
    }
    std::fs::write("kv.db", file_contents).expect("failed to write contents to database")
}
