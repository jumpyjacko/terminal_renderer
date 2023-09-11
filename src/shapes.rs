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

// #[allow(dead_code)]
// pub const PLANE: Object = Object {
//     verts: vec![
//         Float3 {
//             x: 60.0,
//             y: 60.0,
//             z: 0.0,
//         },
//         Float3 {
//             x: 60.0,
//             y: -60.0,
//             z: 0.0,
//         },
//         Float3 {
//             x: -60.0,
//             y: -60.0,
//             z: 0.0,
//         },
//         Float3 {
//             x: -60.0,
//             y: 60.0,
//             z: 0.0,
//         },
//     ],
//     faces: vec![Face { a: 0, b: 1, c: 2 }, Face { a: 0, b: 2, c: 3 }],
// };
//
// #[allow(dead_code)]
// pub const CUBE: Object = Object {
//     verts: vec![
//         // Top face (0-3)
//         Float3 {
//             x: -60.0,
//             y: 60.0,
//             z: -60.0,
//         },
//         Float3 {
//             x: 60.0,
//             y: 60.0,
//             z: -60.0,
//         },
//         Float3 {
//             x: 60.0,
//             y: 60.0,
//             z: 60.0,
//         },
//         Float3 {
//             x: -60.0,
//             y: 60.0,
//             z: 60.0,
//         },
//         // Bottom face (4-7)
//         Float3 {
//             x: -60.0,
//             y: -60.0,
//             z: -60.0,
//         },
//         Float3 {
//             x: 60.0,
//             y: -60.0,
//             z: -60.0,
//         },
//         Float3 {
//             x: 60.0,
//             y: -60.0,
//             z: 60.0,
//         },
//         Float3 {
//             x: -60.0,
//             y: -60.0,
//             z: 60.0,
//         },
//     ],
//     faces: vec![
//         Face { a: 0, b: 1, c: 4 },
//         Face { a: 1, b: 5, c: 4 },
//         Face { a: 1, b: 2, c: 5 },
//         Face { a: 2, b: 6, c: 5 },
//         Face { a: 2, b: 3, c: 6 },
//         Face { a: 3, b: 7, c: 6 },
//         Face { a: 3, b: 0, c: 7 },
//         Face { a: 0, b: 4, c: 7 },
//         Face { a: 0, b: 3, c: 1 },
//         Face { a: 3, b: 2, c: 1 },
//         Face { a: 4, b: 5, c: 7 },
//         Face { a: 5, b: 6, c: 7 },
//     ],
// };
