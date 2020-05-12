
use std::{fs};
use serde::Serialize;

use rocket_contrib::json::JsonValue;

#[derive(Serialize)]
struct File {
    name: String,
    size: u64,
}

// Returns list of files in current directory
#[get("/api/files")]
pub fn files() -> JsonValue {
    let mut results: Vec<File> = vec![];
    
    if let Ok(entries) = fs::read_dir(".") { 
        for entry in entries {        
            if let Ok(entry) = entry {
                let metadata = entry.metadata().unwrap();
                if metadata.is_file() {
                    println!("file: {}", entry.path().display());
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
