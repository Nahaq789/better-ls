#[derive(Debug)]
pub struct Directory<'a> {
    folder_name: &'a str,
    file_name: &'a str,
}

impl<'a> Directory<'a> {
    pub fn new(folder_name: &'a str, file_name: &'a str) -> Self {
        Self {
            folder_name,
            file_name,
        }
    }
    pub fn folder_name(&self) -> &'a str {
        &self.folder_name
    }
    pub fn file_name(&self) -> &'a str {
        &self.file_name
    }
}

impl<'a> std::fmt::Display for Directory<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.file_name)
    }
}
