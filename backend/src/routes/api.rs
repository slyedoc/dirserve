
use std::{fs};
use serde::Serialize;

use rocket_contrib::json::JsonValue;

#[derive(Serialize)]
struct File {
    name: String,
    size: u64,
}

#[get("/api/files")]
pub fn files() -> JsonValue {
    let mut results: Vec<File> = vec![];
    
    //use std::env;
    // let path = env::current_dir().unwrap();
    
    if let Ok(entries) = fs::read_dir(".") { 
        for entry in entries {        
            if let Ok(entry) = entry {
                
                //files.push(File { name: entry.file_name() });
                println!("{:?}", entry.file_name());
                let metadata = entry.metadata().unwrap();

                if metadata.is_file() {
                    results.push(File { 
                        name: entry.file_name().into_string().unwrap(), 
                        size: metadata.len(),
                    });
                }
            }

        }
    }

    json!({ "files": results })
}
