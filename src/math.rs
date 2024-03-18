use crate::parser::ParseError;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Float3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Float2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Int2 {
    pub x: i32,
    pub y: i32,
}

impl Float3 {
    /// Returns the weak projection of a point as Int2
    pub fn project(&self, focal_length: &i32) -> Int2 {
        let p_x: i32 =
            ((*focal_length as f32 * self.x) / (*focal_length as f32 + self.z + 256.0)) as i32;
        let p_y: i32 =
            ((*focal_length as f32 * self.y) / (*focal_length as f32 + self.z + 256.0)) as i32;

        let projected_point: Int2 = Int2 { x: p_x, y: p_y };

        projected_point
    }

    /// Normalises the vector magnitude to 1
    pub fn normalise(&self) -> Float3 {
        let length = (self.x * self.x + self.y * self.y + self.z * self.z).sqrt();
        Float3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    // All three of these rotation functions use the simplified per-axis rotation matrix
    /// Rotate around the x-axis by theta radians
    #[allow(dead_code)]
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

    /// Rotate around the y-axis by theta radians
    #[allow(dead_code)]
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

    /// Rotate around the z-axis by theta radians
    #[allow(dead_code)]
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

    /// Create a new `Float3` from a `Vec<f32>`
    pub fn from_vec(vec_in: Vec<f32>) -> Result<Float3, ParseError> {
        if vec_in.is_empty() {
            return Err(ParseError::EmptyVector);
        }
        Ok(Float3 {
            x: vec_in[0],
            y: vec_in[1],
            z: vec_in[2],
        })
    }
}

impl Int2 {
    /// Returns `bool` if the point `self` is within the triangle given by the three points (`Int2`) provided
    pub fn in_tri(&self, p0: Int2, p1: Int2, p2: Int2) -> bool {
        let fp0 = Float2 { x: p0.x as f32, y: p0.y as f32};
        let fp1 = Float2 { x: p1.x as f32, y: p1.y as f32};
        let fp2 = Float2 { x: p2.x as f32, y: p2.y as f32};

        let s: f32 = fp0.y * fp1.x - fp0.x * fp1.y + (fp1.y - fp0.y) * (self.x as f32) + (fp0.x - fp1.x) * (self.y as f32);
        let t: f32 = fp0.x * fp2.y - fp0.y * fp2.x + (fp0.y - fp2.y) * (self.x as f32) + (fp2.x - fp0.x) * (self.y as f32);

        if s <= 0.0 || t <= 0.0 {
            return false;
        }

        let a = -fp2.y * fp1.x + fp0.y * (-fp2.x + fp1.x) + fp0.x * (fp2.y - fp1.y) + fp2.x * fp1.y;

        (s + t) < a
    }
}

// TODO: Make a "Face" struct (Different from the parsing "Face" struct) and make this an 'impl' of it.
//       Could be a good idea to also place additional face operations there like quad triangulation, etc.
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

    // DEBUG:
    // println!(
    //     "U: {}, {}, {} | V: {}, {}, {} | N: {}, {}, {}",
    //     u.x, u.y, u.z, v.x, v.y, v.z, n.x, n.y, n.z
    // );

    n.normalise()
}
