use std::{env, fs, path::Path, collections::HashSet};
use csv::ReaderBuilder;

#[derive(Debug)]
struct FileSync {
    filename: String,
    local_directory: String,
    sync_directory: String,
    overwrite_allowed: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <absolute_csv_filepath>", args[0]);
        std::process::exit(1);
    }
    
    let csv_path = &args[1];
    if !csv_path.ends_with(".csv") {
        eprintln!("Error: The file must have a .csv extension.");
        std::process::exit(1);
    }
    
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(csv_path).unwrap_or_else(|_| {
        eprintln!("Error: Unable to read the CSV file.");
        std::process::exit(1);
    });
    
    let headers = reader.headers().unwrap().clone();
    if headers.iter().collect::<Vec<_>>() != vec!["filename", "local_directory", "sync_directory", "overwrite_allowed"] {
        eprintln!("Error: CSV headers must be [filename, local_directory, sync_directory, overwrite_allowed]");
        std::process::exit(1);
    }
    
    let mut existing_dirs = HashSet::new();
    let mut valid_files = Vec::new();
    
    for result in reader.records() {
        let record = result.unwrap();
        if record.len() != 4 {
            eprintln!("Error: Invalid number of columns in CSV file.");
            std::process::exit(1);
        }
        
        let filename = record[0].to_string();
        let local_directory = record[1].to_string();
        let sync_directory = record[2].to_string();
        let overwrite_allowed = match record[3].to_lowercase().as_str() {
            "y" => true,
            "n" => false,
            _ => {
                // eprintln!("Warning: Skipping row with invalid overwrite_allowed value: {}", record[3]);
                eprintln!("Warning: Skipping row with invalid overwrite_allowed value");
                continue;
            }
        };
        
        if !dir_exists(&local_directory, &mut existing_dirs) {
            eprintln!("Error: Local directory does not exist: {}", local_directory);
            continue;
        }
        if !dir_exists(&sync_directory, &mut existing_dirs) {
            eprintln!("Error: Sync directory does not exist: {}", sync_directory);
            continue;
        }
        
        valid_files.push(FileSync {
            filename,
            local_directory,
            sync_directory,
            overwrite_allowed,
        });
    }
    
    for file in valid_files {
        let local_filepath = normalize_path(&file.local_directory, &file.filename);
        let sync_filepath = normalize_path(&file.sync_directory, &file.filename);
        copy_file(&local_filepath, &sync_filepath, file.overwrite_allowed);
    }
}

fn dir_exists(path: &str, cache: &mut HashSet<String>) -> bool {
    if cache.contains(path) {
        return true;
    }
    if Path::new(path).is_dir() {
        cache.insert(path.to_string());
        return true;
    }
    false
}

fn normalize_path(directory: &str, filename: &str) -> String {
    let path = Path::new(directory).join(filename);
    path.to_str().unwrap().to_string()
}

fn copy_file(local_filepath: &str, sync_filepath: &str, overwrite_allowed: bool) {
    let local_path = Path::new(local_filepath);
    let sync_path = Path::new(sync_filepath);
    
    if !local_path.exists() {
        eprintln!("Error: Source file does not exist: {}", local_filepath);
        return;
    }
    
    if sync_path.exists() {
        if overwrite_allowed {
            if let Err(e) = fs::copy(local_filepath, sync_filepath) {
                eprintln!("Error: Failed to overwrite file: {} - {}", sync_filepath, e);
            } else {
                println!("Overwritten: {}", sync_filepath);
            }
        } else {
            println!("Skipped (overwrite denied): {}", sync_filepath);
        }
    } else {
        if let Err(e) = fs::copy(local_filepath, sync_filepath) {
            eprintln!("Error: Failed to copy file: {} - {}", sync_filepath, e);
        } else {
            println!("Copied: {}", sync_filepath);
        }
    }
}