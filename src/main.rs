use crate::math::{Point2D, Point3D};
use crate::shapes::Tri;
use std::{thread, time::Duration, time::Instant};

mod math;
mod shapes;

const TILES: [&str; 6] = [" ", ".", ",", "-", "=", "#"];
const SIZE: usize = 30;
const FOCAL_LENGTH: i32 = 30;

const OFFSET: usize = SIZE / 2;

fn main() {
    print!("\x1b[2J\x1b[H");

    let vert_table: Vec<Point3D> = shapes::PLANE.verts.to_vec();
    let tri_table: Vec<Tri> = shapes::PLANE.tris.to_vec();

    let mut theta: f32 = 0.0;

    loop {
        print!("\x1b[H");

        let mut canvas = [[0; SIZE]; SIZE];

        let mut rotated_vert_table: Vec<Point3D> = vec![];
        let mut projected_vert_table: Vec<Point2D> = vec![];

        vert_table.iter().for_each(|v| {
            let rotated_point = v.rotate_y(&theta);
            rotated_vert_table.push(rotated_point);
        });

        rotated_vert_table.iter().for_each(|v| {
            let p_point = v.project(&FOCAL_LENGTH);
            // println!("{}, {}", p_point.x, p_point.y);
            projected_vert_table.push(p_point);
        });

        for y in 0..SIZE {
            for x in 0..SIZE {
                // println!("p: {}, {}", x, y);
                let p: Point2D = Point2D {
                    x: x as i32 - OFFSET as i32,
                    y: y as i32 - OFFSET as i32,
                    // x: x as i32,
                    // y: y as i32,
                };

                for tri in &tri_table {
                    let result: bool = p.in_tri(
                        projected_vert_table[tri.a],
                        projected_vert_table[tri.b],
                        projected_vert_table[tri.c],
                    );

                    // println!(
                    //     "p: {}, {} | tri: ({}, {}), ({}, {}), ({}, {}) | result: {}",
                    //     p.x,
                    //     p.y,
                    //     projected_vert_table[tri.a].x,
                    //     projected_vert_table[tri.a].y,
                    //     projected_vert_table[tri.b].x,
                    //     projected_vert_table[tri.b].y,
                    //     projected_vert_table[tri.c].x,
                    //     projected_vert_table[tri.c].y,
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
