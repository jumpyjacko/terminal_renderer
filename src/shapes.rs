use crate::*;

#[derive(Clone, Copy)]
pub struct Shape<'a> {
    pub verts: &'a [Point3D],
    pub tris: &'a [Tri],
}

#[derive(Clone, Copy)]
pub struct Tri {
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
    tris: &[Tri { a: 0, b: 1, c: 2 }, Tri { a: 0, b: 2, c: 3 }],
    // Back Facing Plane
    // tris: &[Tri { a: 0, b: 2, c: 1 }, Tri { a: 0, b: 3, c: 2 }],
};
