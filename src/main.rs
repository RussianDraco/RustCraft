extern crate piston_window;

use piston_window::*;

fn main() {
    struct Point3D {
        x: f64,
        y: f64,
        z: f64,
    }
    struct Point2D {
        x: f64,
        y: f64,
    }
    struct Vertice {
        start: Point3D,
        end: Point3D,
    }
    struct Cube {
        position: Point3D,
        vertices: Vec<Vertice>,
    }

    fn rotate_x(point: &Point3D, rot: f64) -> Point3D {
        Point3D {
            x: point.x,
            y: f64::cos(rot) * point.y - f64::sin(rot) * point.z,
            z: f64::sin(rot) * point.y + f64::cos(rot) * point.z,
        }
    }
    fn rotate_y(point: &Point3D, rot: f64) -> Point3D {
        Point3D {
            x: f64::cos(rot) * point.x - f64::sin(rot) * point.z,
            y: point.y,
            z: -f64::sin(rot) * point.x + f64::cos(rot) * point.z,
        }
    }
    fn projection(point: &Point3D) -> Point2D {
        Point2D {
            x: WINDOW_X as f64 / 2.0 + (FOV * point.x) / (FOV + point.z) * 100.0,
            y: WINDOW_Y as f64 / 2.0 + (FOV * point.y) / (FOV + point.z) * 100.0,
        }
    }

    const WINDOW_X: i32 = 640;
    const WINDOW_Y: i32 = 480;
    const FOV: f64 = 90.0;

    let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [WINDOW_X as u32, WINDOW_Y as u32])
        .exit_on_esc(true).build().unwrap();

    let mut x = 1.0;
    let mut y = 1.0;
    let mut rotation = 0.0;


    let mut all_cubes: Vec<Cube> = Vec::new();
    let mut cube = Cube {
        position: Point3D { x: 0.0, y: 0.0, z: 0.0 },
        vertices: Vec::new(),
    };
    cube.vertices = vec![
        Vertice {
            start: Point3D { x: -1.0, y: -1.0, z: 1.0 },
            end: Point3D { x: 1.0, y: -1.0, z: 1.0 },
        },
        Vertice {
            start: Point3D { x: 1.0, y: -1.0, z: 1.0 },
            end: Point3D { x: 1.0, y: 1.0, z: 1.0 },
        },
        Vertice {
            start: Point3D { x: 1.0, y: 1.0, z: 1.0 },
            end: Point3D { x: -1.0, y: 1.0, z: 1.0 },
        },
        Vertice {
            start: Point3D { x: -1.0, y: 1.0, z: 1.0 },
            end: Point3D { x: -1.0, y: -1.0, z: 1.0 },
        },
        // Back face
        Vertice {
            start: Point3D { x: -1.0, y: -1.0, z: -1.0 },
            end: Point3D { x: 1.0, y: -1.0, z: -1.0 },
        },
        Vertice {
            start: Point3D { x: 1.0, y: -1.0, z: -1.0 },
            end: Point3D { x: 1.0, y: 1.0, z: -1.0 },
        },
        Vertice {
            start: Point3D { x: 1.0, y: 1.0, z: -1.0 },
            end: Point3D { x: -1.0, y: 1.0, z: -1.0 },
        },
        Vertice {
            start: Point3D { x: -1.0, y: 1.0, z: -1.0 },
            end: Point3D { x: -1.0, y: -1.0, z: -1.0 },
        },
        // Connect front and back faces
        Vertice {
            start: Point3D { x: -1.0, y: -1.0, z: 1.0 },
            end: Point3D { x: -1.0, y: -1.0, z: -1.0 },
        },
        Vertice {
            start: Point3D { x: 1.0, y: -1.0, z: 1.0 },
            end: Point3D { x: 1.0, y: -1.0, z: -1.0 },
        },
        Vertice {
            start: Point3D { x: 1.0, y: 1.0, z: 1.0 },
            end: Point3D { x: 1.0, y: 1.0, z: -1.0 },
        },
        Vertice {
            start: Point3D { x: -1.0, y: 1.0, z: 1.0 },
            end: Point3D { x: -1.0, y: 1.0, z: -1.0 },
        },
    ];
    all_cubes.push(cube);

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            match key {
                Key::W => y -= 1.0,
                Key::A => x -= 1.0,
                Key::S => y += 1.0,
                Key::D => x += 1.0,
                _ => {}
            }
        }

        window.draw_2d(&event, |context, graphics, _| {
            clear([1.0; 4], graphics);
            
            rotation = x * 0.01;

            for cube in all_cubes.iter() {
                for vertice in cube.vertices.iter() {
                    let start = projection(&rotate_x(&rotate_y(&vertice.start, rotation), rotation));
                    let end = projection(&rotate_x(&rotate_y(&vertice.end, rotation), rotation));
                    line([0.0, 0.0, 0.0, 1.0], 1.0, [start.x, start.y, end.x, end.y], context.transform, graphics);
                }
            }
        });
    }
}