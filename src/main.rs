use std::fmt;

fn main() {
    let mut args = std::env::args().skip(1);
    let command = args.next().expect("please provide a command");
    let mut db = Database::new().expect("failed to init database");

    if command == "get_all" {
        print!("{}", db);
        return;
    }

    if command == "get" || command == "set" {
        let key = args.next().expect("please provide a key");
        if command == "get" {
            let value = db.values.get(&key).expect("no value for provided key");
            println!("{}", value);
        } else {
            let value = args.next().expect("no value provided for insert");
            db.insert(key, value);
            db.save();
        }
    } else {
        println!("invalid command");
    }
}

struct Database {
    values: std::collections::HashMap<String, String>,
    did_flush: bool,
}

impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut content = String::new();
        for (key, value) in &self.values {
            let line = format!("{},{}\n", key, value);
            content.push_str(&line);
        }
        write!(f, "{}", content)
    }
}

impl Database {
    fn new() -> Result<Database, std::io::Error> {
        let mut parsed_values: std::collections::HashMap<String, String> =
            std::collections::HashMap::new();
        let file_contents = std::fs::read_to_string("kv.db")?;
        for line in file_contents.lines() {
            match line.split_once(",") {
                Some((key, value)) => {
                    parsed_values.insert(key.to_owned(), value.to_owned());
                }
                None => {
                    return Result::Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "failed to parse db file",
                    ));
                }
            }
        }

        return Result::Ok(Database {
            values: parsed_values,
            did_flush: false,
        });
    }

    fn insert(&mut self, key: String, value: String) {
        self.values.insert(key, value);
    }

    fn save(mut self) {
        if self.did_flush == false {
            self.did_flush = true;
            flush(&self);
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        flush(self);
    }
}

fn flush(db: &Database) {
    let mut file_contents = String::new();
    for (key, value) in &db.values {
        let line = format!("{},{}\n", key, value);
        file_contents.push_str(&line);
    }
    std::fs::write("kv.db", file_contents).expect("failed to write contents to file");
}
