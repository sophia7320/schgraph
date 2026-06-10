use std::fs;

#[derive(Debug)]
struct FileScanner {
    content: Vec<String>
}

impl FileScanner {
    pub fn from_file_path(file_path: &str) -> Self {
        let content = fs::read_to_string(file_path)
                                      .expect("faile to read content from file")
                                      .split_whitespace()
                                                                                .map(String::from)
                                                                                .collect();
        Self { content }
    }
}

impl Iterator for FileScanner {
    fn
}
