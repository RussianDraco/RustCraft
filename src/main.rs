extern crate piston_window;

use piston_window::*;
use std::env;

const WIDTH: u16 = 640;
const HEIGHT: u16 = 480;
const CX: f64 = WIDTH as f64 / 2.0;
const CY: f64 = HEIGHT as f64 / 2.0;
const FOV: f32 = 90.0/180.0*std::f32::consts::PI;
const HALF_FOV: f32 = FOV / 2.0;
const MINZ: f64 = 1.0;

#[derive(Copy, Clone)]
struct Double {
    x: f64,
    y: f64,
}
#[derive(Copy, Clone)]
struct Triple {
    x: f64,
    y: f64,
    z: f64,
}

fn rotate_2d(pos: Double, rot: Double) -> Double {
    Double {
        x: pos.x * rot.y - pos.y * rot.x,
        y: pos.y * rot.y + pos.x * rot.x,
    }
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

    let xz = rotate_2d(Double {x :x, y: z}, cam_rot_y);
    let x = xz.x; let z = xz.y;
    let yz = rotate_2d(Double {x: y, y: z}, cam_rot_x);
    let y = yz.x; let z = yz.y;

    Triple { x: x, y: y, z: z }
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
    fn events(&mut self, event: &Event) {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            let x = f64::sin(self.rot.y);
            let z = f64::cos(self.rot.y);
            match key {
                Key::Up => {
                    self.rot.x -= 0.1;
                }
                Key::Down => {
                    self.rot.x += 0.1;
                }
                Key::Left => {
                    self.rot.y -= 0.1;
                }
                Key::Right => {
                    self.rot.y += 0.1;
                }
                Key::W => {
                    self.pos.x += x;
                    self.pos.z += z;
                }
                Key::S => {
                    self.pos.x -= x;
                    self.pos.z -= z;
                }
                Key::A => {
                    self.pos.x -= z;
                    self.pos.z += x;
                }
                Key::D => {
                    self.pos.x += z;
                    self.pos.z -= x;
                }
                _ => {}
            }
        }
    }

}

//#[derive(Clone)]
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
            [0,1,2,3],
            [4,5,6,7],
            [0,1,5,4],
            [2,3,7,6],
            [0,3,7,4],
            [1,2,6,5]
        ],
        colors: [
            [1.0, 0.0, 0.0, 1.0],
            [0.0, 1.0, 0.0, 1.0],
            [0.0, 0.0, 1.0, 1.0],
            [1.0, 1.0, 0.0, 1.0],
            [1.0, 0.0, 1.0, 1.0],
            [0.0, 1.0, 1.0, 1.0],
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
fn list_of_coords(ve : Vec<Double>) -> [[f64; 2]; 4] {
    let mut arr: [[f64; 2]; 4] = [[0.0; 2]; 4];
    for i in 0..4 {
        arr[i] = [ve[i].x, ve[i].y];
    }
    arr
}

fn calc_depth_vec_helper(verts: Vec<Triple>) -> f64 {
    let mut sum_of_squares = 0.0;

    for i in 0..3 {
        let mut sum_i = 0.0;
        for v in &verts {
            sum_i += triple_index(*v, i) / verts.len() as f64;
        }

        sum_of_squares += sum_i * sum_i;
    }

    sum_of_squares
}

fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    let proj_y: f64 = (CY) / (f64::tan(HALF_FOV as f64) as f64);
    let proj_x: f64 = (CX) / (f64::tan(HALF_FOV as f64) as f64) / (WIDTH as f64 / HEIGHT as f64);

    let mut cam = Cam {
        pos: Triple { x: 0.0, y: 0.0, z: -5.0 },
        rot: Double { x: 0.0, y: 0.0 },
        rot_x: Double { x: 0.0, y: 0.0 },
        rot_y: Double { x: 0.0, y: 0.0 },
    };
    cam.update_rot();

    let pacman_points = [[0,0]];
    let mut cubes: Vec<Cube> = Vec::new();
    for i in pacman_points {cubes.push(init_cube(Triple { x: i[0] as f64, y: 0.0, z: i[1] as f64 }))}

    let mut window: PistonWindow = WindowSettings::new("Rust Craft", [WIDTH as u32, HEIGHT as u32])
        .exit_on_esc(true).build().unwrap();
  

    while let Some(event) = window.next() {
        cam.events(&event);
        cam.update_rot();

        window.draw_2d(&event, |context, graphics, _| {
            clear([1.0; 4], graphics);

            let mut face_list: Vec<Vec<Double>> = Vec::new();
            let mut face_color: Vec<[f32; 4]> = Vec::new();
            let mut depth: Vec<f64> = Vec::new();

            for obj in cubes.iter_mut() {
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
                            i += (sides.len() as i32 - 1);
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

                polygon(color, &list_of_coords(face.to_vec()), context.transform, graphics);
            }

            });
    }
}