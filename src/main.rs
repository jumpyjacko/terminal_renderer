use crate::math::{Float3, Int2};
use crate::shapes::Face;
use std::{thread, time::Duration, time::Instant};

mod math;
mod shapes;

const TILES: [&str; 6] = [" ", ".", ",", "-", "=", "#"];
const SIZE: usize = 30;
const FOCAL_LENGTH: i32 = 30;

const LIGHT_VECTOR: Float3 = Float3 {
    x: -1.0,
    y: -1.0,
    z: -1.0,
};

const OFFSET: usize = SIZE / 2;

fn main() {
    print!("\x1b[2J\x1b[H");

    let vert_table: Vec<Float3> = shapes::PLANE.verts.to_vec();
    let face_table: Vec<Face> = shapes::PLANE.faces.to_vec();

    let mut theta: f32 = 0.0;

    loop {
        print!("\x1b[H");

        let mut canvas = [[0; SIZE]; SIZE];

        let mut rotated_vert_table: Vec<Float3> = vec![];
        let mut projected_vert_table: Vec<Int2> = vec![];

        vert_table.iter().for_each(|v| {
            let rotated_point = v.rotate_y(&theta);
            rotated_vert_table.push(rotated_point);
        });

        rotated_vert_table.iter().for_each(|v| {
            let p_point = v.project(&FOCAL_LENGTH);
            // println!("{}, {}", p_point.x, p_point.y);
            projected_vert_table.push(p_point);
        });

        // Face Drawing
        for y in 0..SIZE {
            for x in 0..SIZE {
                // println!("p: {}, {}", x, y);
                let p: Int2 = Int2 {
                    x: x as i32 - OFFSET as i32,
                    y: y as i32 - OFFSET as i32,
                    // x: x as i32,
                    // y: y as i32,
                };

                for face in &face_table {
                    let result: bool = p.in_tri(
                        projected_vert_table[face.a],
                        projected_vert_table[face.b],
                        projected_vert_table[face.c],
                    );

                    // println!(
                    //     "p: {}, {} | tri: ({}, {}), ({}, {}), ({}, {}) | result: {}",
                    //     p.x,
                    //     p.y,
                    //     projected_vert_table[face.a].x,
                    //     projected_vert_table[face.a].y,
                    //     projected_vert_table[face.b].x,
                    //     projected_vert_table[face.b].y,
                    //     projected_vert_table[face.c].x,
                    //     projected_vert_table[face.c].y,
                    //     result
                    // );
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

        theta += 0.05;

        thread::sleep(Duration::from_millis(30))
    }
}
