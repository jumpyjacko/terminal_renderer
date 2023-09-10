use crate::Float3;

#[derive(Clone, Copy, Debug)]
pub struct Object<'a> {
    pub verts: &'a [Float3],
    pub faces: &'a [Face],
}

#[derive(Clone, Copy, Debug)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

#[allow(dead_code)]
pub const PLANE: Object = Object {
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

#[allow(dead_code)]
pub const CUBE: Object = Object {
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
        Face { a: 0, b: 1, c: 4 },
        Face { a: 1, b: 5, c: 4 },
        Face { a: 1, b: 2, c: 5 },
        Face { a: 2, b: 6, c: 5 },
        Face { a: 2, b: 3, c: 6 },
        Face { a: 3, b: 7, c: 6 },
        Face { a: 3, b: 0, c: 7 },
        Face { a: 0, b: 4, c: 7 },
        Face { a: 0, b: 3, c: 1 },
        Face { a: 3, b: 2, c: 1 },
        Face { a: 4, b: 5, c: 7 },
        Face { a: 5, b: 6, c: 7 },
    ],
};
