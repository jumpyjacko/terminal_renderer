use crate::math::{calculate_face_normal, Float3, Int2};
use crate::shapes::Face;
use std::{thread, time::Duration, time::Instant};

mod math;
mod shapes;

const TILES: [&str; 6] = [" ", ".", ",", "-", "=", "#"];
const SIZE: usize = 30;
const FOCAL_LENGTH: i32 = 30;

const OFFSET: usize = SIZE / 2;

fn main() {
    print!("\x1b[2J\x1b[H");

    let vert_table: Vec<Float3> = shapes::PLANE.verts.to_vec();
    let face_table: Vec<Face> = shapes::PLANE.faces.to_vec();

    let light_vector: Float3 = Float3 {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    }
    .normalise();

    let mut theta: f32 = 0.0;

    loop {
        print!("\x1b[H");

        let mut canvas = [[0; SIZE]; SIZE];

        let mut transformed_vert_table: Vec<Float3> = vec![];
        let mut projected_vert_table: Vec<Int2> = vec![];

        vert_table.iter().for_each(|v| {
            let transformed_point = v.rotate_y(&theta);
            transformed_vert_table.push(transformed_point);
        });

        transformed_vert_table.iter().for_each(|v| {
            let p_point = v.project(&FOCAL_LENGTH);
            projected_vert_table.push(p_point);
        });

        // Normal Calculations
        for face in &face_table {
            let normal = calculate_face_normal(
                transformed_vert_table[face.a],
                transformed_vert_table[face.b],
                transformed_vert_table[face.c],
            );

            let dot_product =
                normal.x * light_vector.x + normal.y * light_vector.y + normal.z * light_vector.z;
        }

        // Face Drawing
        for y in 0..SIZE {
            for x in 0..SIZE {
                let p: Int2 = Int2 {
                    x: x as i32 - OFFSET as i32,
                    y: y as i32 - OFFSET as i32,
                };

                for face in &face_table {
                    let result: bool = p.in_tri(
                        projected_vert_table[face.a],
                        projected_vert_table[face.b],
                        projected_vert_table[face.c],
                    );

                    if result {
                        canvas[y][x] = 1;
                    }
                }
            }
        }

        for row in canvas.iter_mut() {
            for cell in row.iter_mut() {
                print!("{}{0}", TILES[*cell]);
            }
            println!();
        }

        // theta += 0.05;

        thread::sleep(Duration::from_millis(30))
    }
}
