use crate::*;

#[derive(Clone, Copy)]
pub struct Shape<'a> {
    pub verts: &'a [Point3D],
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
        Point3D {
            x: 60.0,
            y: 60.0,
            z: 0.0,
        },
        Point3D {
            x: 60.0,
            y: -60.0,
            z: 0.0,
        },
        Point3D {
            x: -60.0,
            y: -60.0,
            z: 0.0,
        },
        Point3D {
            x: -60.0,
            y: 60.0,
            z: 0.0,
        },
    ],
    // Front Facing Plane
    faces: &[Face { a: 0, b: 1, c: 2 }, Tri { a: 0, b: 2, c: 3 }],
    // Back Facing Plane
    // faces: &[Face { a: 0, b: 2, c: 1 }, Tri { a: 0, b: 3, c: 2 }],
};
