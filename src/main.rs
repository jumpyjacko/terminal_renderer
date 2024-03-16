use crate::math::{calculate_face_normal, Float3, Int2};
use crate::parser::from_obj;
use crate::shapes::Face;
use std::time::{Duration, Instant};
use std::thread;
use std::env;

mod math;
mod parser;
mod shapes;

const TILES: [&str; 10] = [" ", ".", ":", "-", "=", "+", "*", "#", "%", "@"];
const SIZE: usize = 60;
const FOCAL_LENGTH: i32 = 60;

const OFFSET: usize = SIZE / 2;

fn main() {

    // let vert_table: Vec<Float3> = shapes::CUBE.verts.to_vec();
    // let face_table: Vec<Face> = shapes::CUBE.faces.to_vec();
    let args: Vec<String> = env::args().collect();

    let object_path = match args[1].len() < 2 {
        true => panic!("No path to an .obj file provided"),
        false => args[1].to_owned(),
    };
    
    let loaded_object = match from_obj(&object_path, 100.0) {
        Ok(o) => o,
        Err(e) => panic!("Failed with error: {:?}", e),
    };
    let vert_table: Vec<Float3> = loaded_object.verts;
    let face_table: Vec<Face> = loaded_object.faces;

    let light_vector: Float3 = Float3 {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    }
    .normalise();

    let mut theta: f32 = 0.0;

    print!("\x1b[2J\x1b[H");
    loop {
        print!("\x1b[H");
        let frame_timer = Instant::now();

        let mut canvas = [[0; SIZE]; SIZE];

        let mut transformed_vert_table: Vec<Float3> = vec![];
        let mut projected_vert_table: Vec<Int2> = vec![];
        let mut face_shading_lut: Vec<usize> = vec![];

        vert_table.iter().for_each(|v| {
            let transformed_point = v.rotate_y(&theta).rotate_z(&theta);
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

            let dot_product: f32 =
                normal.x * light_vector.x + normal.y * light_vector.y + normal.z * light_vector.z;

            face_shading_lut.push((((dot_product + 1.0) / 2.0) * 10.0) as usize);
        }

        // Face Drawing
        for y in 0..SIZE {
            for x in 0..SIZE {
                let p: Int2 = Int2 {
                    x: x as i32 - OFFSET as i32,
                    y: y as i32 - OFFSET as i32,
                };

                for (i, face) in face_table.iter().enumerate() {
                    let result: bool = p.in_tri(
                        projected_vert_table[face.a],
                        projected_vert_table[face.b],
                        projected_vert_table[face.c],
                    );

                    if result {
                        canvas[y][x] = face_shading_lut[i];
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

        // TODO: Cap frame rate to 60 fps, needs to properly work with deltatime calculations
        let frame_time = match frame_timer.elapsed().as_millis() {
            0 => 1, // Hardcoded prevention from dividing by 0
            millis => millis,
        };
        
        theta += 0.0005 * frame_timer.elapsed().as_millis() as f32;
        
        println!("Calculation Duration: {} ms", frame_time);
        println!("   Frames per Second: {}", 1000 / frame_time);
        
        // if frame_time < 16 {
        //     thread::sleep(Duration::from_millis(15 - frame_time as u64));
        // }
    }
}
