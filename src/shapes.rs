use crate::*;

#[derive(Clone, Copy)]
pub struct Shape<'a> {
    pub verts: &'a [Float3],
    pub faces: &'a [Face],
}

#[derive(Clone, Copy)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

pub const PLANE: Shape = Shape {
    verts: &[
        Float3 {
            x: 60.0,
            y: 60.0,
            z: 0.0,
        },
        Float3 {
            x: 60.0,
            y: -60.0,
            z: 0.0,
        },
        Float3 {
            x: -60.0,
            y: -60.0,
            z: 0.0,
        },
        Float3 {
            x: -60.0,
            y: 60.0,
            z: 0.0,
        },
    ],
    faces: &[Face { a: 0, b: 1, c: 2 }, Face { a: 0, b: 2, c: 3 }],
};
