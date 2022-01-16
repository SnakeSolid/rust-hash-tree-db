#[macro_use]
extern crate log;

mod command;

use crate::command::Command;
use hash_tree_db::Config;
use hash_tree_db::Database;
use hash_tree_db::PrintVisiter;
use rustyline::error::ReadlineError;
use rustyline::Editor;

#[global_allocator]
static ALLOC: jemallocator::Jemalloc = jemallocator::Jemalloc;

type Data = String;

fn main() {
    env_logger::init();

    info!("starting up");

    let config = Config::default()
        .set_max_page_size(10)
        .set_max_pages(Some(2))
        .set_storage_path("storage");
    let mut database: Database<Data, Data, Data> = Database::new(config);
    let mut editor = Editor::<()>::new();

    loop {
        let input = match editor.readline(">> ") {
            Ok(line) => line,
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);

                break;
            }
        };

        match Command::parse(&input) {
            Ok(Command::Get { hash_key, tree_key }) => {
                editor.add_history_entry(&input);

                let hash_key = hash_key.into();
                let tree_key = tree_key.into();

                match database.get(&hash_key, &tree_key) {
                    Ok(Some(data)) => println!("OK {} {} {}", hash_key, tree_key, data),
                    Ok(None) => println!("OK"),
                    Err(error) => println!("ERR {}", error),
                }
            }
            Ok(Command::Put {
                hash_key,
                tree_key,
                data,
            }) => {
                editor.add_history_entry(&input);

                let hash_key = hash_key.into();
                let tree_key = tree_key.into();
                let data = data.into();

                match database.put(hash_key, tree_key, data) {
                    Ok(_replaced) => println!("OK"),
                    Err(error) => println!("ERR {}", error),
                }
            }
            Ok(Command::Contains { hash_key, tree_key }) => {
                editor.add_history_entry(&input);

                let hash_key = hash_key.into();
                let tree_key = tree_key.into();

                match database.contains(&hash_key, &tree_key) {
                    Ok(true) => println!("OK TRUE"),
                    Ok(false) => println!("OK FALSE"),
                    Err(error) => println!("ERR {}", error),
                }
            }
            Ok(Command::Delete { hash_key, tree_key }) => {
                editor.add_history_entry(&input);

                let hash_key = hash_key.into();
                let tree_key = tree_key.into();

                match database.delete(&hash_key, &tree_key) {
                    Ok(true) => println!("OK TRUE"),
                    Ok(false) => println!("OK FALSE"),
                    Err(error) => println!("ERR {}", error),
                }
            }
            Ok(Command::Count {}) => {
                editor.add_history_entry(&input);

                match database.count() {
                    Ok(count) => println!("OK {}", count),
                    Err(error) => println!("ERR {}", error),
                }
            }
            Ok(Command::Show {}) => {
                editor.add_history_entry(&input);

                database.visit(&mut PrintVisiter::default());

                println!("OK");
            }
            Ok(Command::Exit {}) => {
                break;
            }
            Ok(command) => {
                println!("Unknown command: {:?}", command);
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}
