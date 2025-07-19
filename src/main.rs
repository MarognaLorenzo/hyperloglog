mod hyperloglog;
use hyperloglog::HyperLogLog;
use std::{io};

fn main() {
    println!("HyperLogLog Word Counter");
    println!("Choose an option:");
    println!("1. Interactive mode - enter words manually");
    println!("2. File mode - process words from a file");
    println!("Enter your choice (1 or 2):");
    
    let mut choice = String::new();
    match io::stdin().read_line(&mut choice) {
        Ok(_) => {
            let choice = choice.trim();
            match choice {
                "1" => {
                    let mut structure = HyperLogLog::new("interactive mode".to_string());
                    user_feedback(&mut structure);
                }
                "2" => {
                    use std::fs;
                    use std::path::Path;
                    
                    let resources_dir = "resources";
                    
                    // Check if resources directory exists
                    if !Path::new(resources_dir).exists() {
                        println!("Resources directory not found. Please create a 'resources' folder with .txt files.");
                        return;
                    }
                    
                    // Read directory and filter for .txt files
                    match fs::read_dir(resources_dir) {
                        Ok(entries) => {
                            let mut txt_files: Vec<String> = Vec::new();
                            
                            for entry in entries {
                                if let Ok(entry) = entry {
                                    let path = entry.path();
                                    if let Some(extension) = path.extension() {
                                        if extension == "txt" {
                                            if let Some(file_name) = path.file_name() {
                                                if let Some(file_name_str) = file_name.to_str() {
                                                    txt_files.push(file_name_str.to_string());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            
                            if txt_files.is_empty() {
                                println!("No .txt files found in the resources directory.");
                                return;
                            }
                            
                            // Display available files
                            println!("Available .txt files in resources:");
                            for (index, file) in txt_files.iter().enumerate() {
                                println!("{}. {}", index + 1, file);
                            }
                            
                            println!("Enter the number of the file you want to process:");
                            
                            let mut file_choice = String::new();
                            match io::stdin().read_line(&mut file_choice) {
                                Ok(_) => {
                                    let file_choice = file_choice.trim();
                                    match file_choice.parse::<usize>() {
                                        Ok(choice) => {
                                            if choice > 0 && choice <= txt_files.len() {
                                                let selected_file = &txt_files[choice - 1];
                                                let full_path = format!("{}/{}", resources_dir, selected_file);
                                                println!("Selected file: {}", selected_file);
                                                from_file(full_path);
                                            } else {
                                                println!("Invalid choice. Please enter a number between 1 and {}.", txt_files.len());
                                            }
                                        }
                                        Err(_) => {
                                            println!("Invalid input. Please enter a number.");
                                        }
                                    }
                                }
                                Err(error) => {
                                    println!("Error reading input: {}", error);
                                }
                            }
                        }
                        Err(error) => {
                            println!("Error reading resources directory: {}", error);
                        }
                    }
                }
                _ => {
                    println!("Invalid choice. Please run the program again and enter 1 or 2.");
                }
            }
        }
        Err(error) => {
            println!("Error reading input: {}", error);
        }
    }
}
pub fn from_file(file: String) {
    use std::fs;
use std::collections::HashSet;
    
    let mut structure = HyperLogLog::new("file processing".to_string());
    
    match fs::read_to_string(&file) {
        Ok(contents) => {
            println!("Processing file: {}", file);
            
            // Split the file contents into words and process each one
            let words: Vec<&str> = contents
                .split_whitespace()
                .collect();
            
            println!("Found {} words in the file", words.len());
            let mut set: HashSet<String> = HashSet::new();
            
            for word in words {
                // Clean the word (remove punctuation, convert to lowercase)
                let cleaned_word = word
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .collect::<String>()
                    .to_lowercase();
                
                if !cleaned_word.is_empty() {
                    set.insert(cleaned_word.clone());
                    structure.receive(cleaned_word);
                }
            }
            
            println!("Finished processing file.");
            println!("Expected unique words: {}", structure.get_single_words());
            println!("Unique words: {}", set.len());
        }
        Err(error) => {
            println!("Error reading file '{}': {}", file, error);
        }
    }
}
pub fn user_feedback(structure: &mut HyperLogLog) {
    println!("HyperLogLog initialized! Enter strings to process (type 'quit' to exit):");
    
    loop {
        println!("\nEnter a string (type `quit` to exit):");
        
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_string();
                
                if input == "quit" {
                    println!("Goodbye!");
                    break;
                }
                
                if !input.is_empty() {
                    structure.receive(input);
                } else {
                    println!("Please enter a non-empty string.");
                }
            }
            Err(error) => {
                println!("Error reading input: {}", error);
            }
        }
    }

    println!("Expected unique words: {}", structure.get_single_words());
}
