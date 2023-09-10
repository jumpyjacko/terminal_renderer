#[derive(Clone, Copy, Debug)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct Int2 {
    pub x: i32,
    pub y: i32,
}

impl Float3 {
    pub fn project(&self, focal_length: &i32) -> Int2 {
        let p_x: i32 =
            ((*focal_length as f32 * self.x) / (*focal_length as f32 + self.z + 256.0)) as i32;
        let p_y: i32 =
            ((*focal_length as f32 * self.y) / (*focal_length as f32 + self.z + 256.0)) as i32;

        let projected_point: Int2 = Int2 { x: p_x, y: p_y };

        projected_point
    }

    pub fn normalise(&self) -> Float3 {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Float3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    // All three of these rotation functions use the simplified per-axis rotation matrix
    pub fn rotate_x(&self, theta: &f32) -> Float3 {
        let yz = &[self.y, self.z];
        let matrix = &[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]];

        let rotated_yz = &[
            matrix[0][0] * yz[0] + matrix[0][1] * yz[1],
            matrix[1][0] * yz[0] + matrix[1][1] * yz[1],
        ];

        Float3 {
            x: self.x,
            y: rotated_yz[0],
            z: rotated_yz[1],
        }
    }

    pub fn rotate_y(&self, theta: &f32) -> Float3 {
        let xz = &[self.x, self.z];
        let matrix = &[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]];

        let rotated_xz = &[
            matrix[0][0] * xz[0] + matrix[0][1] * xz[1],
            matrix[1][0] * xz[0] + matrix[1][1] * xz[1],
        ];

        Float3 {
            x: rotated_xz[0],
            y: self.y,
            z: rotated_xz[1],
        }
    }

    pub fn rotate_z(&self, theta: &f32) -> Float3 {
        let xy = &[self.x, self.y];
        let matrix = &[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]];

        let rotated_xy = &[
            matrix[0][0] * xy[0] + matrix[0][1] * xy[1],
            matrix[1][0] * xy[0] + matrix[1][1] * xy[1],
        ];

        Float3 {
            x: rotated_xy[0],
            y: rotated_xy[1],
            z: self.z,
        }
    }
}

impl Int2 {
    pub fn in_tri(&self, p0: Int2, p1: Int2, p2: Int2) -> bool {
        let side_1 = (self.x - p1.x) * (p0.y - p1.y) - (p0.x - p1.x) * (self.y - p1.y);
        let side_2 = (self.x - p2.x) * (p1.y - p2.y) - (p1.x - p2.x) * (self.y - p2.y);
        let side_3 = (self.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (self.y - p0.y);

        return (side_1 < 0) && (side_2 < 0) && (side_3 < 0);
    }
}

// TODO: Make a "Face" struct and make this an 'impl' of it
pub fn calculate_face_normal(p0: Float3, p1: Float3, p2: Float3) -> Float3 {
    let u = Float3 {
        x: p1.x - p0.x,
        y: p1.y - p0.y,
        z: p1.z - p0.z,
    };

    let v = Float3 {
        x: p2.x - p0.x,
        y: p2.y - p0.y,
        z: p2.z - p0.z,
    };

    let n = Float3 {
        x: u.y * v.z - u.z * v.y,
        y: u.z * v.x - u.x * v.z,
        z: u.x * v.y - u.y * v.x,
    };

    // println!(
    //     "U: {}, {}, {} | V: {}, {}, {} | N: {}, {}, {}",
    //     u.x, u.y, u.z, v.x, v.y, v.z, n.x, n.y, n.z
    // );

    n.normalise()
}
