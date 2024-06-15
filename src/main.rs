extern crate piston_window;
extern crate noise;

use noise::{NoiseFn, Perlin, Seedable};
use piston_window::*;
use std::env;
use std::time::{Duration, Instant};
use std::collections::HashMap;

mod generation;
mod texture;

const WIDTH: u16 = 640;
const HEIGHT: u16 = 480;
const CX: f64 = WIDTH as f64 / 2.0;
const CY: f64 = HEIGHT as f64 / 2.0;
const FOV: f32 = 90.0/180.0*std::f32::consts::PI * 0.75;
const HALF_FOV: f32 = FOV / 2.0;
const MINZ: f64 = 0.1;
pub const CHUNK_RENDER_DIST: u8 = 1; //chunk size is 16x16
pub const SEED: u32 = 91224;
const TARGET_FPS: u32 = 45;

#[derive(Copy, Clone)]
pub struct Double {
    x: f64,
    y: f64,
}
#[derive(Copy, Clone)]
pub struct Triple {
    x: f64,
    y: f64,
    z: f64,
}

fn get_2d(v: Triple, proj_y: f64, proj_x: f64) -> Double {
    Double {
        x: CX + (v.x / v.z * proj_x),
        y: CY + (v.y / v.z * proj_y),
    }
}
fn get_3d(v: Triple, cam_pos: Triple, cam_rot_y: Double, cam_rot_x: Double) -> Triple {
    let x = v.x - cam_pos.x;
    let y = v.y - cam_pos.y;
    let z = v.z - cam_pos.z;

    let xz_x = x * cam_rot_y.y - z * cam_rot_y.x;
    let xz_z = z * cam_rot_y.y + x * cam_rot_y.x;
    let yz_y = y * cam_rot_x.y - xz_z * cam_rot_x.x;
    let yz_z = xz_z * cam_rot_x.y + y * cam_rot_x.x;

    Triple { x: xz_x, y: yz_y, z: yz_z }
}

fn get_z(a: Triple, b: Triple, new_z: f64) -> Triple {
    if a.z == b.z || new_z < a.z || new_z > b.z {
        return Triple { x: 0.0, y: 0.0, z: 0.0 };
    }

    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let dz = b.z - a.z;

    let i = (new_z - a.z) / dz;

    Triple {
        x: a.x + dx * i,
        y: a.y + dy * i,
        z: new_z,
    }
}

struct Cam {
    pos: Triple,
    rot: Double,
    rot_x: Double,
    rot_y: Double,
    disp: Double,
    keys: [bool; 8],
}
impl Cam {
    fn update_rot(&mut self) {
        self.rot_x = Double {
            x: f64::sin(self.rot.x),
            y: f64::cos(self.rot.x),
        };
        self.rot_y = Double {
            x: f64::sin(self.rot.y),
            y: f64::cos(self.rot.y),
        };
    }
    fn events(&mut self, event: &Event, dt: f64) {    
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W => self.keys[0] = true,
                Key::A => self.keys[1] = true,
                Key::S => self.keys[2] = true,
                Key::D => self.keys[3] = true,
                Key::Up => self.keys[4] = true,
                Key::Down => self.keys[5] = true,
                Key::Left => self.keys[6] = true,
                Key::Right => self.keys[7] = true,
                _ => (),
            }
        }
        if let Some(Button::Keyboard(key)) = event.release_args() {
            match key {
                Key::W => self.keys[0] = false,
                Key::A => self.keys[1] = false,
                Key::S => self.keys[2] = false,
                Key::D => self.keys[3] = false,
                Key::Up => self.keys[4] = false,
                Key::Down => self.keys[5] = false,
                Key::Left => self.keys[6] = false,
                Key::Right => self.keys[7] = false,
                _ => (),
            }
        }
        
        let s = 10.0 * dt;
        let ss = 2.0 * dt;
        let x = s * f64::sin(self.rot.y);
        let z = s * f64::cos(self.rot.y);
        if self.keys[0] {
            self.pos.x += x;
            self.pos.z += z;
            self.disp.x += x;
            self.disp.y += z;
        }
        if self.keys[2] {
            self.pos.x -= x;
            self.pos.z -= z;
            self.disp.x -= x;
            self.disp.y -= z;
        }
        if self.keys[1] {
            self.pos.x -= z;
            self.pos.z += x;
            self.disp.x -= z;
            self.disp.y += x;
        }
        if self.keys[3] {
            self.pos.x += z;
            self.pos.z -= x;
            self.disp.x += z;
            self.disp.y -= x;
        }
        if self.keys[4] {
            self.rot.x -= ss;
        }
        if self.keys[5] {
            self.rot.x += ss;
        }
        if self.keys[6] {
            self.rot.y -= ss;
        }
        if self.keys[7] {
            self.rot.y += ss;
        }

        self.rot.x = self.rot.x.max(-1.5).min(1.5);
        self.rot.y = self.rot.y % (std::f64::consts::PI * 2.0);

        self.pos.x = (self.pos.x * 100.0).round() / 100.0;
        self.pos.y = (self.pos.y * 100.0).round() / 100.0;
        self.pos.z = (self.pos.z * 100.0).round() / 100.0;
        self.rot.x = (self.rot.x * 100.0).round() / 100.0;
        self.rot.y = (self.rot.y * 100.0).round() / 100.0;
    }

}

struct Cube {
    //pos: Triple,
    verts: [Triple; 8],
    faces: [[u8; 4]; 6],
    colors: [[f32; 4]; 6],
}
fn init_cube(p: Triple) -> Cube{
    let _verts = [
        Triple { x: -0.5 + p.x, y: -0.5 + p.y, z: -0.5 + p.z },
        Triple { x: 0.5 + p.x, y: -0.5 + p.y, z: -0.5 + p.z },
        Triple { x: 0.5 + p.x, y: 0.5 + p.y, z: -0.5 + p.z },
        Triple { x: -0.5 + p.x, y: 0.5 + p.y, z: -0.5 + p.z },
        Triple { x: -0.5 + p.x, y: -0.5 + p.y, z: 0.5 + p.z },
        Triple { x: 0.5 + p.x, y: -0.5 + p.y, z: 0.5 + p.z },
        Triple { x: 0.5 + p.x, y: 0.5 + p.y, z: 0.5 + p.z },
        Triple { x: -0.5 + p.x, y: 0.5 + p.y, z: 0.5 + p.z },
    ];

    Cube {
        //pos: p,
        verts: _verts,
        faces: [
            [0,1,2,0],
            [0,5,6,0],
            [0,1,5,0],
            [0,3,7,0],
            [0,3,7,0],
            [0,2,6,0]
        ],
        colors: [
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
        ]
    }
}
fn triple_index(triple: Triple, index: u8) -> f64 {
    match index {
        0 => triple.x,
        1 => triple.y,
        2 => triple.z,
        _ => 0.0,
    }
}
fn list_of_coords(ve: Vec<Double>) -> Vec<[f64; 2]> {
    let mut arr: Vec<[f64; 2]> = vec![[0.0; 2]; ve.len()];
    for i in 0..ve.len() {
        arr[i] = [ve[i].x, ve[i].y];
    }
    arr
}
fn calc_depth_vec_helper(verts: Vec<Triple>) -> f64 {
    let mut sum_of_squares = 0.0;

    for i in 0..3 {
        let sum_i: f64 = verts.iter().map(|v| triple_index(*v, i) / verts.len() as f64).sum();
        sum_of_squares += sum_i * sum_i;
    }

    sum_of_squares
}



fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    let proj_y: f64 = (CY) / (f64::tan(HALF_FOV as f64) as f64);
    let proj_x: f64 = (CX) / (f64::tan(HALF_FOV as f64) as f64) / (WIDTH as f64 / HEIGHT as f64);

    let mut cam = Cam {
        pos: Triple { x: 0.0, y: -1.0, z: -5.0 },
        rot: Double { x: 0.0, y: 0.0 },
        rot_x: Double { x: 0.0, y: 0.0 },
        rot_y: Double { x: 0.0, y: 0.0 },
        disp: Double { x: 0.0, y: 0.0 },
        keys: [false; 8],
    };
    cam.update_rot();

    let mut window: PistonWindow = WindowSettings::new("Rust Craft", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true).build().unwrap();

    let perlin_generator = Perlin::new().set_seed(SEED);
    let mut cubes: Vec<Cube> = generation::generate_cubes(cam.pos, perlin_generator);
    let frame_duration = Duration::from_secs_f64(1.0 / TARGET_FPS as f64);
    let mut previous_frame_time = Instant::now();
    let textures: Vec<[[[f32; 4]; 16]; 16]> = texture::texture_dict();

    while let Some(event) = window.next() {
        let current_frame_time = Instant::now();
        let dt = (current_frame_time - previous_frame_time).as_secs_f64();
        previous_frame_time = current_frame_time;
        cam.events(&event, dt);
        cam.update_rot();

        if (cam.disp.x > generation::CHUNK_SIZE as f64 || cam.disp.x < -generation::CHUNK_SIZE as f64) || (cam.disp.y > generation::CHUNK_SIZE as f64 || cam.disp.y < -generation::CHUNK_SIZE as f64) {
            cubes = generation::generate_cubes(cam.pos, perlin_generator);
            cam.disp = Double { x: 0.0, y: 0.0 };
        }

        window.draw_2d(&event, |context, graphics, _| {
            clear([1.0; 4], graphics);

            let mut face_list: Vec<Vec<Double>> = Vec::new();
            let mut face_color: Vec<[f32; 4]> = Vec::new();
            let mut depth: Vec<f64> = Vec::new();

            for obj in cubes.iter() {
                let mut vert_list: [Triple; 8] = [Triple { x: 0.0, y: 0.0, z: 0.0 }; 8];
                for n in 0..8 {
                    vert_list[n] = get_3d(obj.verts[n], cam.pos, cam.rot_y, cam.rot_x);
                }

                for f in 0..6 {
                    let mut verts: Vec<Triple> = Vec::new();
                    for n in 0..4 {
                        verts.push(vert_list[obj.faces[f][n] as usize]);
                    }

                    let mut i: i32 = 0;
                    while i < verts.len() as i32 {
                        if verts[i as usize].z < MINZ {
                            let mut sides: Vec<Triple> = Vec::new();
                            let l = verts[{
                                if i == 0 {
                                    verts.len() - 1
                                } else {
                                    (i - 1) as usize
                                }
                            }];
                            let r = verts[((i as usize + 1) % verts.len()) as usize];
                            if l.z > MINZ {
                                sides.push(get_z(verts[i as usize], l, MINZ));
                            }
                            if r.z > MINZ {
                                sides.push(get_z(verts[i as usize], r, MINZ));
                            }
                            
                            verts = verts[..(i as usize)].iter().chain(sides.iter()).chain(verts[((i+1) as usize)..].iter()).cloned().collect();
                            i += sides.len() as i32 - 1;
                        }
                        i += 1;
                    }

                    if verts.len() > 2 {
                        let mut face_list_push: Vec<Double> = Vec::new();
                        for v in verts.clone() {
                            face_list_push.push(get_2d(v, proj_y, proj_x));
                        }
                        face_list.push(face_list_push);

                        face_color.push(obj.colors[f]);

                        depth.push(calc_depth_vec_helper(verts));
                    }
                }
            }

            let mut order: Vec<usize> = (0..face_list.len()).collect();
            order.sort_by(|&i, &j| depth[j].partial_cmp(&depth[i]).unwrap());

            for i in order {
                let face = &face_list[i];
                let color = face_color[i];
                let mut texture: [[[f32; 4]; 16]; 16];
                let mut use_texture = false;
                if color[3] == 0.0 {
                    texture = textures[color[0] as usize];
                    use_texture = true;
                }

                if use_texture {
                    polygon(color, &list_of_coords(face.to_vec()), context.transform, graphics);
                }
                else {
                    polygon(color, &list_of_coords(face.to_vec()), context.transform, graphics);
                }

                for i in 0..face.len() {
                    let j = (i + 1) % face.len();
                    line([0.0, 0.0, 0.0, 1.0], 1.0, [face[i].x, face[i].y, face[j].x, face[j].y], context.transform, graphics);
                }
            }

            println!("x: {}, y: {}, z: {}", cam.pos.x, cam.pos.y, cam.pos.z);
        });

        let elapsed_time = current_frame_time.elapsed();
        if elapsed_time < frame_duration {
            std::thread::sleep(frame_duration - elapsed_time);
        }
    }
}