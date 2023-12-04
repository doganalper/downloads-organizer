use std::{collections::HashSet, env, fs, process};
use whoami::username;

fn main() {
    let path = find_os_path();

    let dir_paths = get_dir_content(&path);
    let dir_names = get_dir_names(&path, &dir_paths);

    create_dirs(dir_names);
    move_files(path, &dir_paths);
}

fn move_files(path: String, dir_paths: &Vec<String>) {
    for file_path in dir_paths {
        let folder_name = get_folder_by_extension(file_path);

        if folder_name.eq("") == false {
            let file_name = file_path.split("/").last().unwrap();
            let new_path = format!("{}/{}/{}", path, folder_name, file_name);
            match fs::rename(file_path, new_path) {
                Ok(_) => println!("{file_name} moved to {folder_name}"),
                Err(_) => println!("There was an error with {file_name}"),
            }
        }
    }
}

fn create_dirs(dir_names: HashSet<String>) {
    for dir in dir_names {
        if fs::create_dir(&dir).is_ok() {
            println!("Created {dir}")
        }
    }
}

fn get_folder_by_extension(file_path: &str) -> String {
    let file_extension = file_path.split(".").last().unwrap_or("").to_lowercase();
    String::from(match file_extension.as_str() {
        "png" | "jpg" | "jpeg" | "webp" | "gif" | "svg" | "heic" => "Images",
        "mp4" | "avi" | "mov" => "Videos",
        "pdf" | "doc" | "docx" | "txt" | "ppt" | "pptx" | "csv" => "Documents",
        _ => "",
    })
}

fn get_dir_names(path: &String, dir_paths: &Vec<String>) -> HashSet<String> {
    let mut dir_names = HashSet::new();
    for file_path in dir_paths {
        let folder_name = get_folder_by_extension(file_path);

        if folder_name.eq("") {
            continue;
        }

        let full_path = format!("{}/{}", path, folder_name);
        dir_names.insert(full_path);
    }

    dir_names
}

fn get_dir_content(path: &String) -> Vec<String> {
    let dir = fs::read_dir(path);

    match dir {
        Ok(value) => {
            let mut paths = vec![];
            for path in value {
                let dir_entry = &path.unwrap();
                let path = &dir_entry.path();
                let display = &path.display();
                let display_str = display.to_string();
                paths.push(display_str);
            }

            paths
        }
        Err(err) => {
            println!("There was an error reading path: {err}");
            process::exit(1)
        }
    }
}

fn find_os_path() -> String {
    let os = env::consts::OS;
    let user_name = username();

    let os_path = match os {
        "macos" => format!("/Users/{}/Downloads", user_name),
        "windows" => format!("C:\\users\\{}\\downloads", user_name),
        _ => {
            println!("Not supported OS being used. Feel free to contribute!");
            process::exit(1)
        }
    };

    os_path
}
