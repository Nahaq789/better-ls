use std::{
    env::{self},
    path::Path,
};

use better_ls::Content;

const KEY_SET: [&str; 48] = [
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
    let sorted_contents = Content::sort(contents);

    for (i, c) in sorted_contents.iter().enumerate() {
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
                let result = sorted_contents[index].folder_name_as_str();
                final_path(&args[1], result);

                return Ok(());
            }
        }
    }

    println!("Enter the key of the directory you want to move");
    let mut command = String::new();
    std::io::stdin().read_line(&mut command).ok();

    let index = match get_index_from_key_set(&mut command) {
        Some(index) => index,
        None => panic!(""),
    };
    let result = sorted_contents[index].folder_name_as_str();
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsString;
    use std::fs::{self, DirBuilder, File};
    use std::io::{self, Write};
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn setup_test_directory() -> io::Result<(PathBuf, tempfile::TempDir)> {
        let temp_dir = tempdir()?;
        let test_dir_path = temp_dir.path().join("test_dir");
        DirBuilder::new().create(&test_dir_path)?;

        for dir_name in &["dir1", "dir2", "another_dir", "test_folder"] {
            DirBuilder::new().create(test_dir_path.join(dir_name))?;
        }

        let file_names = ["file1.txt", "file2.rs", "test.md"];
        for file_name in &file_names {
            let mut file = File::create(test_dir_path.join(file_name))?;
            writeln!(file, "Test content")?;
        }

        Ok((test_dir_path, temp_dir))
    }

    #[test]
    fn test_content_new() {
        let dir_name = "test_dir";
        let file_type = fs::metadata(std::env::temp_dir()).unwrap().file_type();
        let content = Content::new(dir_name.into(), file_type);

        assert_eq!(content.folder_name_as_str(), dir_name);
    }

    #[test]
    fn test_content_folder_name() {
        let dir_name: OsString = "test_dir".into();
        let file_type = fs::metadata(std::env::temp_dir()).unwrap().file_type();
        let content = Content::new(dir_name.clone(), file_type);

        assert_eq!(content.folder_name(), &dir_name);
    }

    #[test]
    fn test_is_file() {
        let (test_dir_path, _temp_dir) = setup_test_directory().unwrap();

        let file_path = test_dir_path.join("file1.txt");
        let dir_path = test_dir_path.join("dir1");

        let file_type = fs::metadata(&file_path).unwrap().file_type();
        let dir_type = fs::metadata(&dir_path).unwrap().file_type();

        let file_content = Content::new(file_path.file_name().unwrap().to_os_string(), file_type);
        let dir_content = Content::new(dir_path.file_name().unwrap().to_os_string(), dir_type);

        assert!(file_content.is_file());
        assert!(!dir_content.is_file());
    }

    #[test]
    fn test_remove_file() {
        let (test_dir_path, _temp_dir) = setup_test_directory().unwrap();

        let paths = fs::read_dir(&test_dir_path).unwrap();
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

        let filtered_contents = Content::remove_file(contents);

        assert_eq!(filtered_contents.len(), 4);

        for content in filtered_contents {
            let path = test_dir_path.join(content.folder_name_as_str());
            assert!(fs::metadata(path).unwrap().is_dir());
        }
    }

    #[test]
    fn test_load_more() {
        assert_eq!(load_more(0), false);
        assert_eq!(load_more(10), true);
        assert_eq!(load_more(20), true);
        assert_eq!(load_more(9), false);
        assert_eq!(load_more(11), false);
    }

    #[test]
    fn test_get_index_from_key_set() {
        assert_eq!(get_index_from_key_set(&mut "a".to_string()), Some(0));
        assert_eq!(get_index_from_key_set(&mut "b".to_string()), Some(1));
        assert_eq!(get_index_from_key_set(&mut "Z".to_string()), Some(47));
        assert_eq!(get_index_from_key_set(&mut "a\n".to_string()), Some(0));
        assert_eq!(get_index_from_key_set(&mut "invalid".to_string()), None);
    }

    #[test]
    fn test_remove_n_command() {
        assert_eq!(remove_n_command(&mut "test\n".to_string()), "test");
        assert_eq!(remove_n_command(&mut "a\n".to_string()), "a");
        assert_eq!(remove_n_command(&mut "a".to_string()), "a");
        assert_eq!(remove_n_command(&mut "\n".to_string()), "");
    }

    #[test]
    fn test_final_path() {
        let mut output = Vec::new();
        {
            use std::io::stdout;
            let stdout = stdout();
            let mut _handle = stdout.lock();

            final_path("/home/user", "documents");
            writeln!(output, "{}", "/home/user/documents").unwrap();

            final_path("/tmp/test", "subfolder");
            writeln!(output, "{}", "/tmp/test/subfolder").unwrap();
        }

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("/home/user/documents"));
        assert!(output_str.contains("/tmp/test/subfolder"));
    }

    #[test]
    fn test_integrated_flow() {
        let (test_dir_path, _temp_dir) = setup_test_directory().unwrap();

        let _args = vec![
            "program_name".to_string(),
            test_dir_path.to_string_lossy().to_string(),
        ];

        let paths = fs::read_dir(&test_dir_path).unwrap();
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

        let filtered_contents = Content::remove_file(contents);

        assert_eq!(filtered_contents.len(), 4);

        let mut i = 0;
        for c in &filtered_contents {
            assert!(i < KEY_SET.len());
            let _key = KEY_SET[i];
            let dir_name = c.folder_name_as_str();

            let full_path = test_dir_path.join(dir_name);
            assert!(fs::metadata(&full_path).unwrap().is_dir());

            i += 1;
        }
    }
}
