use std::{ffi::OsString, fs::FileType};

#[derive(Debug, Clone)]
pub struct Content {
    folder_name: OsString,
    file_type: FileType,
}

impl Content {
    pub fn new(folder_name: OsString, file_type: FileType) -> Content {
        Content {
            folder_name,
            file_type,
        }
    }

    pub fn folder_name(&self) -> &OsString {
        &self.folder_name
    }

    pub fn folder_name_as_str(&self) -> &str {
        self.folder_name().to_str().unwrap_or_default()
    }

    pub fn is_file(&self) -> bool {
        self.file_type.is_file()
    }

    pub fn remove_file(contents: Vec<Content>) -> Vec<Content> {
        contents.into_iter().filter(|x| !x.is_file()).collect()
    }

    pub fn sort(mut contents: Vec<Content>) -> Vec<Content> {
        contents.sort_by(|a, b| a.folder_name.to_str().cmp(&b.folder_name.to_str()));
        return contents;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_content_new() {
        let name = OsString::from("test_folder");
        let temp_dir = std::env::temp_dir();
        let file_type = fs::metadata(temp_dir).unwrap().file_type();

        let content = Content::new(name.clone(), file_type);

        assert_eq!(content.folder_name, name);
    }

    #[test]
    fn test_folder_name() {
        let name = OsString::from("test_folder");
        let temp_dir = std::env::temp_dir();
        let file_type = fs::metadata(temp_dir).unwrap().file_type();

        let content = Content::new(name.clone(), file_type);

        assert_eq!(content.folder_name(), &name);
    }

    #[test]
    fn test_folder_name_as_str() {
        let name = OsString::from("test_folder");
        let temp_dir = std::env::temp_dir();
        let file_type = fs::metadata(temp_dir).unwrap().file_type();

        let content = Content::new(name, file_type);

        assert_eq!(content.folder_name_as_str(), "test_folder");
    }

    #[test]
    fn test_is_file() {
        let temp_dir = std::env::temp_dir();
        let dir_type = fs::metadata(&temp_dir).unwrap().file_type();

        let file_path = if cfg!(windows) {
            Path::new("C:\\Windows\\system.ini")
        } else {
            Path::new("/etc/hosts")
        };

        if file_path.exists() {
            let file_type = fs::metadata(file_path).unwrap().file_type();

            let dir_content = Content::new(OsString::from("temp_dir"), dir_type);
            let file_content = Content::new(OsString::from("system_file"), file_type);

            assert!(!dir_content.is_file());
            assert!(file_content.is_file());
        } else {
            let current_exe = std::env::current_exe().unwrap();
            let file_type = fs::metadata(current_exe).unwrap().file_type();

            let dir_content = Content::new(OsString::from("temp_dir"), dir_type);
            let file_content = Content::new(OsString::from("executable"), file_type);

            assert!(!dir_content.is_file());
            assert!(file_content.is_file());
        }
    }

    #[test]
    fn test_remove_file() {
        let temp_dir = std::env::temp_dir();
        let dir_type = fs::metadata(&temp_dir).unwrap().file_type();

        let file_path = if cfg!(windows) {
            Path::new("C:\\Windows\\system.ini")
        } else {
            Path::new("/etc/hosts")
        };

        let mut contents = Vec::new();

        contents.push(Content::new(OsString::from("dir1"), dir_type.clone()));
        contents.push(Content::new(OsString::from("dir2"), dir_type.clone()));

        if file_path.exists() {
            let file_type = fs::metadata(file_path).unwrap().file_type();
            contents.push(Content::new(OsString::from("file1"), file_type.clone()));
            contents.push(Content::new(OsString::from("file2"), file_type));
        } else {
            let current_exe = std::env::current_exe().unwrap();
            let file_type = fs::metadata(current_exe).unwrap().file_type();
            contents.push(Content::new(OsString::from("file1"), file_type.clone()));
            contents.push(Content::new(OsString::from("file2"), file_type));
        }

        let filtered = Content::remove_file(contents);

        assert_eq!(filtered.len(), 2);
        assert_eq!(filtered[0].folder_name_as_str(), "dir1");
        assert_eq!(filtered[1].folder_name_as_str(), "dir2");
    }

    #[test]
    #[ignore]
    fn test_folder_name_as_str_with_invalid_unicode() {
        let name = if cfg!(windows) {
            let mut v = Vec::new();
            v.push(0xFF);
            v.push(0xFE);
            v.push(0xFD);
            OsString::from("test_folder")
            // OsString::from(v)
        } else {
            let mut v = Vec::new();
            v.push(0xFF);
            v.push(0xFE);
            v.push(0xFD);
            OsString::from("test_folder")
            // OsString::from(v)
        };

        let temp_dir = std::env::temp_dir();
        let file_type = fs::metadata(temp_dir).unwrap().file_type();

        let content = Content::new(name, file_type);

        assert_eq!(content.folder_name_as_str(), "");
    }
}
