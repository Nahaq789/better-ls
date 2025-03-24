use std::{
    env::{self},
    path::Path,
    usize,
};

use better_ls::Content;

const KEY_SET: [&'static str; 48] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u",
    "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "L", "M", "N", "O", "P",
    "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
];

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];
    let paths = std::fs::read_dir(path)?;

    let contents: Vec<Content> = paths
        .filter_map(|result| {
            result.ok().and_then(|dir_entry| {
                dir_entry
                    .file_type()
                    .ok()
                    .map(|f| Content::new(dir_entry.file_name(), f))
            })
        })
        .collect();
    let contents = Content::remove_file(contents);

    let mut i = 0;

    for c in &contents {
        println!("key: {}  {}", KEY_SET[i], c.folder_name_as_str());
        if load_more(i) {
            println!("load more...");
            let mut command = String::new();
            std::io::stdin().read_line(&mut command).ok();
            if !command.eq("\n") {
                let index = match get_index_from_key_set(&mut command) {
                    Some(index) => index,
                    None => panic!(""),
                };
                let result = contents[index].folder_name_as_str();
                final_path(&args[1], result);

                return Ok(());
            }
        }
        i += 1;
    }

    println!("Enter the key of the directory you want to move");
    let mut command = String::new();
    std::io::stdin().read_line(&mut command).ok();

    let index = match get_index_from_key_set(&mut command) {
        Some(index) => index,
        None => panic!(""),
    };
    let result = contents[index].folder_name_as_str();
    final_path(&args[1], result);

    Ok(())
}

fn final_path(args: &str, result: &str) {
    let base = Path::new(args);
    let sub = Path::new(result);

    println!("{}", base.join(sub).display())
}

fn load_more(index: usize) -> bool {
    if index % 10 == 0 && !index.eq(&0) {
        return true;
    }
    false
}

fn get_index_from_key_set(command: &mut str) -> Option<usize> {
    let command = remove_n_command(command);
    for (index, &item) in KEY_SET.iter().enumerate() {
        if item == command {
            return Some(index);
        }
    }
    None
}

fn remove_n_command(command: &mut str) -> String {
    command.to_string().replace("\n", "")
}
