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
        match self.folder_name.to_str() {
            Some(s) => s,
            None => "",
        }
    }

    pub fn is_file(&self) -> bool {
        return self.file_type.is_file();
    }

    pub fn remove_file(contents: Vec<Content>) -> Vec<Content> {
        return contents.into_iter().filter(|x| !x.is_file()).collect();
    }
}
