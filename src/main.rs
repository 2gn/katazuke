// use std::env::current_dir;
use std::io;
// use std::path::Path;

use std::fs::{read_dir, remove_dir_all};

fn main() {
    let mut death_note = Vec::new();
    if let Ok(list) = read_dir(".") {
        for entry in list {
            if let Ok(file_or_folder) = entry {
                if file_or_folder
                    .file_type()
                    .expect("could not infer file type")
                    .is_dir()
                {
                    let stdin = io::stdin();
                    let mut buffer = String::new();
                    println!(
                        "Do you want to remove this project?: {}",
                        file_or_folder.path().to_str().unwrap()
                    );
                    stdin
                        .read_line(&mut buffer)
                        .expect("could not read user input");
                    if "buffer" == "y\n" {
                        death_note.push(file_or_folder.path());
                    }
                }
            }
        }
    }
    for victim in death_note {
        let victim_name = &victim.to_str().unwrap();
        match remove_dir_all(&victim) {
            Ok(_e) => {
                println!("Successfully removed the directory: {}", victim_name);
            },
            Err(_e) => {
                println!("Failed to remove the directory: {}", victim_name);
            }
        }
    }
}
