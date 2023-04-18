use ndarray::arr1;
use ndarray::arr2;

#[derive(Clone, Copy)]
pub struct Point3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Clone, Copy)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point3D {
    pub fn project(&self, focal_length: &i32) -> Point2D {
        let p_x: i32 =
            ((*focal_length as f32 * self.x) / (*focal_length as f32 + self.z + 256.0)) as i32;
        let p_y: i32 =
            ((*focal_length as f32 * self.y) / (*focal_length as f32 + self.z + 256.0)) as i32;

        let projected_point: Point2D = Point2D { x: p_x, y: p_y };

        projected_point
    }

    // All three of these rotation functions use the simplified per-axis rotation matrix
    pub fn rotate_x(&self, theta: &f32) -> Point3D {
        let yz = arr1(&[self.y, self.z]);
        let matrix = arr2(&[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]]);

        let rotated_yz = matrix.dot(&yz);

        Point3D {
            x: self.x,
            y: rotated_yz[0],
            z: rotated_yz[1],
        }
    }

    pub fn rotate_y(&self, theta: &f32) -> Point3D {
        let xz = arr1(&[self.x, self.z]);
        let matrix = arr2(&[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]]);

        let rotated_xz = matrix.dot(&xz);

        Point3D {
            x: rotated_xz[0],
            y: self.y,
            z: rotated_xz[1],
        }
    }

    pub fn rotate_z(&self, theta: &f32) -> Point3D {
        let xy = arr1(&[self.x, self.y]);
        let matrix = arr2(&[[theta.cos(), -theta.sin()], [theta.sin(), theta.cos()]]);

        let rotated_xy = matrix.dot(&xy);

        Point3D {
            x: rotated_xy[0],
            y: rotated_xy[1],
            z: self.z,
        }
    }
}

impl Point2D {
    pub fn in_tri(&self, p0: Point2D, p1: Point2D, p2: Point2D) -> bool {
        // let d_x = self.x - p2.x;
        // let d_y = self.y - p2.y;
        // let d_x21 = p2.x - p1.x;
        // let d_y12 = p1.y - p2.y;
        // let d = d_y12 * (p0.x - p2.x) + d_x21 * (p0.y - p2.y);
        // let s = d_y12 * d_x + d_x21 * d_y;
        // let t = d_x * (p2.y - p0.y) + d_y * (p0.x - p2.x);
        // if d < 0 {
        //     return s <= 0 && t <= 0 && s + t <= d;
        // }
        // return s >= 0 && t >= 0 && s + t <= d;

        // let a = 1 / 2 * (-p1.y * p2.x + p0.y * (-p1.x + p2.x) + p0.x * (p1.y - p2.y) + p1.x * p2.y);
        // let sign = if a < 0 { -1 } else { 1 };
        // let s =
        //     (p0.y * p2.x - p0.x * p2.y + (p2.y - p0.y) * self.x + (p0.x - p2.x) * self.y) * sign;
        // let t =
        //     (p0.x * p1.y - p0.y * p1.x + (p0.y - p1.y) * self.x + (p1.x - p0.x) * self.y) * sign;
        //
        // return s > 0 && t > 0 && (s + t) < 2 * a * sign;

        let side_1 = (self.x - p1.x) * (p0.y - p1.y) - (p0.x - p1.x) * (self.y - p1.y);
        let side_2 = (self.x - p2.x) * (p1.y - p2.y) - (p1.x - p2.x) * (self.y - p2.y);
        let side_3 = (self.x - p0.x) * (p2.y - p0.y) - (p2.x - p0.x) * (self.y - p0.y);

        return (side_1 < 0) && (side_2 < 0) && (side_3 < 0);
    }
}
