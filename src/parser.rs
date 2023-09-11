use crate::{shapes::Object, Face, Float3};
use std::fs;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
    FailedToRead,
    EmptyVector,
}

/// Creates an 'Object' type from a '.obj' file
pub fn from_obj(path: &str) -> Result<Object, ParseError> {
    let file: String = read_file(path)?;

    let mut verts: Vec<Float3> = Vec::new();
    let mut faces: Vec<Face> = Vec::new();

    for line in file.lines() {
        if line.is_empty() {
            continue;
        }

        let mut words: Vec<&str> = line.split_whitespace().collect();

        if words[0] == "v" {
            words.remove(0);
            let words = words
                .into_iter()
                .map(|e| e.parse::<f32>().expect("Failed to parse vertice to float"))
                .collect();

            verts.push(match Float3::from_vec(words) {
                Ok(v) => v,
                Err(e) => panic!("Failed with error: {:?}", e),
            });
        } else if words[0] == "f" {
            words.remove(0);
            let words: Vec<usize> = words
                .into_iter()
                .map(|e| e.parse::<usize>().expect("Failed to parse face to usize") - 1)
                .collect();
            if words.len() == 4 {
                faces.push(match Face::from_vec(vec![words[0], words[1], words[2]]) {
                    Ok(f) => f,
                    Err(e) => panic!("Failed with error: {:?}", e),
                });
                faces.push(match Face::from_vec(vec![words[1], words[2], words[3]]) {
                    Ok(f) => f,
                    Err(e) => panic!("Failed with error: {:?}", e),
                });
            } else {
                faces.push(match Face::from_vec(vec![words[0], words[1], words[2]]) {
                    Ok(f) => f,
                    Err(e) => panic!("Failed with error: {:?}", e),
                });
            }
        }
    }

    Ok(Object { verts, faces })
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

    #[test]
    fn test_read_obj_file() {
        let result = match from_obj("./tests/box.obj") {
            Ok(r) => r,
            Err(e) => panic!("u fucked up bro: {:?}", e),
        };
        assert_eq!(
            result,
            Object {
                verts: vec![
                    Float3 {
                        x: -0.5,
                        y: -0.5,
                        z: 0.5
                    },
                    Float3 {
                        x: -0.5,
                        y: -0.5,
                        z: -0.5
                    },
                    Float3 {
                        x: -0.5,
                        y: 0.5,
                        z: -0.5
                    },
                    Float3 {
                        x: -0.5,
                        y: 0.5,
                        z: 0.5
                    },
                    Float3 {
                        x: 0.5,
                        y: -0.5,
                        z: 0.5
                    },
                    Float3 {
                        x: 0.5,
                        y: -0.5,
                        z: -0.5
                    },
                    Float3 {
                        x: 0.5,
                        y: 0.5,
                        z: -0.5
                    },
                    Float3 {
                        x: 0.5,
                        y: 0.5,
                        z: 0.5
                    },
                ],
                faces: vec![
                    Face { a: 3, b: 2, c: 1 },
                    Face { a: 2, b: 1, c: 0 },
                    Face { a: 1, b: 5, c: 4 },
                    Face { a: 5, b: 4, c: 0 },
                    Face { a: 2, b: 6, c: 6 },
                    Face { a: 6, b: 5, c: 1 },
                    Face { a: 7, b: 6, c: 2 },
                    Face { a: 6, b: 2, c: 3 },
                    Face { a: 4, b: 7, c: 3 },
                    Face { a: 7, b: 3, c: 0 },
                    Face { a: 5, b: 6, c: 7 },
                    Face { a: 6, b: 7, c: 4 },
                ],
            }
        );
    }
}
