use crate::{shapes::Object, Face, Float3};
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    FailedToRead,
    FailedToParse,
}

/// Creates an 'Object' type from a '.obj' file
pub fn from_obj(path: &str) -> Result<Object<'static>, ParseError> {
    let file: String = read_file(path)?;

    let verts: Vec<Float3> = Vec::new();
    let faces: Vec<Face> = Vec::new();

    for line in file.lines() {
        if line.is_empty() {
            continue;
        }

        let words: Vec<&str> = line.split_whitespace().collect();

        // Extracting the verts
        if words[0] == "v" {
            todo!();
        }
    }

    todo!()
}

pub fn read_file(path: &str) -> Result<String, ParseError> {
    match fs::read_to_string(path) {
        Ok(f) => Ok(f),
        Err(_) => return Err(ParseError::FailedToRead),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_success() {
        let result = read_file("./tests/test.txt");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Contents of test.txt\n");
    }

    #[test]
    fn test_read_file_failure() {
        let result = read_file("./tests/nonexistent.txt");
        match result {
            Ok(_) => panic!("Expected an error, but got Ok"),
            Err(e) => assert_eq!(e, ParseError::FailedToRead),
        }
    }

    // #[test]
    // fn test_read_obj_file() {
    //     let result = from_obj("box.obj");
    //     assert_eq!(result, Object { ... });
    // }
}
