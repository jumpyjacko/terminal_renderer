use crate::parser::ParseError;
use crate::Float3;

#[derive(Clone, Debug, PartialEq)]
pub struct Object {
    pub verts: Vec<Float3>,
    pub faces: Vec<Face>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Face {
    /// Creates a `Face` struct from a `Vec<usize>`
    pub fn from_vec(vec_in: Vec<usize>) -> Result<Face, ParseError> {
        if vec_in.is_empty() {
            return Err(ParseError::EmptyVector);
        }
        Ok(Face {
            a: vec_in[0],
            b: vec_in[1],
            c: vec_in[2],
        })
    }
}
