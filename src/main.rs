use std::fs::{read_dir, metadata, remove_dir_all, remove_file};
use std::path::{Path, PathBuf};
use std::io::{stdin,stdout,Write};
use std::process::exit;

fn asker(path: PathBuf) -> bool {
    let mut s=String::new();

    print!("Remove {:?} ? (Y/N): ", path);

    let _=stdout().flush();

    stdin().read_line(&mut s).expect("Did not enter a correct string");

    if s == "Y\r\n" || s == "y\r\n" {
        return true;
    } else if s == "q\r\n" {
        println!("Aborted");
        exit(0);
    } else {
        return false;
    }
}


fn main() {
    let dir_iter = read_dir(Path::new(".")).expect("could not read the current directory");

    let mut death_list = Vec::new();

    for item in dir_iter {
        match item {
            Ok(dir_entry) => {
                if asker(dir_entry.path()) {
                    death_list.push(dir_entry.path());
                }
            },
            Err(_) => {
                println!("Errored");
            }
        }
    }

    for item in death_list {
        let filetype = metadata(item.clone()).expect("failed to read metadata").file_type();

        if filetype.is_dir() {
            match remove_dir_all(&item) {
                Ok(_) => {
                    println!("successfully removed {:?}", &item)
                },
                Err(_) => {
                    println!("Failed to remove {:?}", &item)
                }
            }
        } else if filetype.is_file() {
            match remove_file(&item) {
                Ok(_) => {
                    println!("successfully removed {:?}", &item)
                },
                Err(_) => {
                    println!("Failed to remove {:?}", &item)
                }
            }
        }
    }
}
