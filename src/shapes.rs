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

pub const CUBE: Shape = Shape {
    verts: &[
        // Top face (0-3)
        Float3 {
            x: -60.0,
            y: 60.0,
            z: -60.0,
        },
        Float3 {
            x: 60.0,
            y: 60.0,
            z: -60.0,
        },
        Float3 {
            x: 60.0,
            y: 60.0,
            z: 60.0,
        },
        Float3 {
            x: -60.0,
            y: 60.0,
            z: 60.0,
        },
        // Bottom face (4-7)
        Float3 {
            x: -60.0,
            y: -60.0,
            z: -60.0,
        },
        Float3 {
            x: 60.0,
            y: -60.0,
            z: -60.0,
        },
        Float3 {
            x: 60.0,
            y: -60.0,
            z: 60.0,
        },
        Float3 {
            x: -60.0,
            y: -60.0,
            z: 60.0,
        },
    ],
    faces: &[
        // Front face (0-1-5-4)
        Face { a: 0, b: 1, c: 4 },
        Face { a: 1, b: 5, c: 4 },
        // Right face (1-2-6-5)
        Face { a: 1, b: 2, c: 5 },
        Face { a: 2, b: 6, c: 5 },
        // Back face (2-3-7-6)
        Face { a: 2, b: 3, c: 6 },
        Face { a: 3, b: 7, c: 6 },
        // Left face (3-0-4-7)
        Face { a: 3, b: 0, c: 7 },
        Face { a: 0, b: 4, c: 7 },
        // Top face (0-3-2-1)
        Face { a: 0, b: 3, c: 1 },
        Face { a: 3, b: 2, c: 1 },
        // Bottom face (4-5-6-7)
        Face { a: 4, b: 5, c: 7 },
        Face { a: 5, b: 6, c: 7 },
    ],
};
