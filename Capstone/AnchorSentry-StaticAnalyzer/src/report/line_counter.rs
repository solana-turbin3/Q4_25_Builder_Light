use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
pub struct FileInfo {
    pub comments_count: i32,
    pub blank_spaces_count: i32,
    pub lines_of_code: i32,
}

impl FileInfo {
    pub fn new() -> Self {
        FileInfo {
            comments_count: 0,
            blank_spaces_count: 0,
            lines_of_code: 0,
        }
    }
}

/// Analyze a single file and return counts of comments, blank lines, and code lines.
pub fn analyze_file<P: AsRef<Path>>(path: P) -> std::io::Result<FileInfo> {
    let mut file = File::open(&path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    let contents = String::from_utf8_lossy(&contents);
    let mut info = FileInfo::new();
    let mut in_multiline_comment = false;

    for line in contents.lines() {
        let trimmed = line.trim();

        if in_multiline_comment {
            info.comments_count += 1;
            if trimmed.ends_with("*/")
            {
                in_multiline_comment = false;
            }
        } else if trimmed.starts_with("//")
            || trimmed.starts_with("//!")
            || trimmed.starts_with("///")
        {
            info.comments_count += 1;
        } else if trimmed.starts_with("/*") {
            info.comments_count += 1;
            //multiline comment in single line
            if !trimmed.ends_with("*/") {
                in_multiline_comment = true;
            }
        } else if trimmed.is_empty() {
            info.blank_spaces_count += 1;
        } else {
            info.lines_of_code += 1;
        }
    }

    Ok(info)
}

pub fn count_lines(path: String) {
    let file_path = path;
    match analyze_file(&file_path) {
        Ok(info) => {
            println!("File: {}", file_path);
            println!("Lines of code: {}", info.lines_of_code);
            println!("Comments: {}", info.comments_count);
            println!("Blank spaces: {}", info.blank_spaces_count);
            println!("Total lines in file: {}", info.lines_of_code + info.comments_count + info.blank_spaces_count)
        }
        Err(e) => println!("Error reading file: {}", e),
    }
}
