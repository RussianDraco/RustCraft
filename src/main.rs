extern crate piston_window;

use piston_window::*;
use std::collections::HashSet;

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

    fn projection(point: &Point3D) -> Point2D {
        Point2D {
            x: FOV as f64 * point.x / point.z + (WINDOW_X as f64 / 2.0),
            y: FOV as f64 * point.y / point.z + (WINDOW_Y as f64 / 2.0),
        }
    }
    fn rotate_2d(point: &Point2D, angle: f64) -> Point2D {
        Point2D {
            x: point.x * f64::cos(angle) - point.y * f64::sin(angle),
            y: point.x * f64::sin(angle) + point.y * f64::cos(angle),
        }
    }
    fn rotate_3d(point: &Point3D, camxrot: f64, camyrot: f64) -> Point3D{
        let rot2d = rotate_2d(&Point2D {x: point.z, y: point.x}, camxrot);
        let outx = rot2d.y;
        let rot2d2 = rotate_2d(&Point2D {x: point.y, y: rot2d.x}, camyrot);
        Point3D {
            x: outx,
            y: rot2d2.x,
            z: rot2d2.y,
        }
    }
    fn draw_line(p1: &Point3D, p2: &Point3D, x: f64, y: f64, z: f64, camxrot: f64, camyrot: f64, context: Context, graphics: &mut G2d) {
        let mut p1 = rotate_3d(&Point3D { x: p1.x+x, y: p1.y+y, z: p1.z+z }, camxrot, camyrot);
        let mut p2 = rotate_3d(&Point3D { x: p2.x+x, y: p2.y+y, z: p2.z+z }, camxrot, camyrot);

        if p1.z > ZCLIPDIST || p2.z > ZCLIPDIST {
            //zclip
            let zcpercent = (ZCLIPDIST - p1.z) / (p2.z - p1.z);
            if p1.z < ZCLIPDIST {
                p1.x = p1.x + (p2.x - p1.x) * zcpercent;
                p1.y = p1.y + (p2.y - p1.y) * zcpercent;
                p1.z = ZCLIPDIST;
            }
            if p2.z < ZCLIPDIST {
                p2.x = p1.x + (p2.x - p1.x) * zcpercent;
                p2.y = p1.y + (p2.y - p1.y) * zcpercent;
                p2.z = ZCLIPDIST;
            }

            let p1 = projection(&p1);
            let p2 = projection(&p2);

            line([0.0, 0.0, 0.0, 1.0], 0.5, [p1.x, p1.y, p2.x, p2.y], context.transform, graphics);
        }
    }
    fn draw_plane(p1: &Point3D, p2: &Point3D, p3: &Point3D, p4: &Point3D, x: f64, y: f64, z: f64, camxrot: f64, camyrot: f64, context: Context, graphics: &mut G2d) {
        let p1 = rotate_3d(&Point3D { x: p1.x+x, y: p1.y+y, z: p1.z+z }, camxrot, camyrot);
        let p2 = rotate_3d(&Point3D { x: p2.x+x, y: p2.y+y, z: p2.z+z }, camxrot, camyrot);
        let p3 = rotate_3d(&Point3D { x: p3.x+x, y: p3.y+y, z: p3.z+z }, camxrot, camyrot);
        let p4 = rotate_3d(&Point3D { x: p4.x+x, y: p4.y+y, z: p4.z+z }, camxrot, camyrot);

        let p1 = projection(&p1);
        let p2 = projection(&p2);
        let p3 = projection(&p3);
        let p4 = projection(&p4);

        polygon([0.0, 1.0, 0.0, 1.0], &[[p1.x, p1.y], [p2.x, p2.y], [p3.x, p3.y], [p4.x, p4.y]], context.transform, graphics);
    }
    fn cube(cube_x: f64, cube_y: f64, cube_z: f64, x: f64, y: f64, z: f64, camxrot: f64, camyrot: f64, context: Context, graphics: &mut G2d) {
        /*draw_plane(&Point3D {x: cube_x + 50.0, y: cube_y + 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y + 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y - 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x + 50.0, y: cube_y - 50.0, z: cube_z - 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_plane(&Point3D {x: cube_x + 50.0, y: cube_y + 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y + 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y - 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x + 50.0, y: cube_y - 50.0, z: cube_z + 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_plane(&Point3D {x: cube_x + 50.0, y: cube_y + 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x + 50.0, y: cube_y + 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x + 50.0, y: cube_y - 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x + 50.0, y: cube_y - 50.0, z: cube_z - 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_plane(&Point3D {x: cube_x - 50.0, y: cube_y + 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y + 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y - 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y - 50.0, z: cube_z - 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_plane(&Point3D {x: cube_x + 50.0, y: cube_y + 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y + 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y + 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x + 50.0, y: cube_y + 50.0, z: cube_z + 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_plane(&Point3D {x: cube_x + 50.0, y: cube_y - 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y - 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y - 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x + 50.0, y: cube_y - 50.0, z: cube_z + 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        **/

        draw_line(&Point3D {x: cube_x + 50.0, y: cube_y + 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y + 50.0, z: cube_z - 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_line(&Point3D {x: cube_x + 50.0, y: cube_y - 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y - 50.0, z: cube_z - 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_line(&Point3D {x: cube_x + 50.0, y: cube_y + 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x + 50.0, y: cube_y - 50.0, z: cube_z - 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_line(&Point3D {x: cube_x - 50.0, y: cube_y + 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y - 50.0, z: cube_z - 50.0}, x, y, z, camxrot, camyrot, context, graphics);

        draw_line(&Point3D {x: cube_x + 50.0, y: cube_y + 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y + 50.0, z: cube_z + 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_line(&Point3D {x: cube_x + 50.0, y: cube_y - 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y - 50.0, z: cube_z + 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_line(&Point3D {x: cube_x + 50.0, y: cube_y + 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x + 50.0, y: cube_y - 50.0, z: cube_z + 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_line(&Point3D {x: cube_x - 50.0, y: cube_y + 50.0, z: cube_z + 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y - 50.0, z: cube_z + 50.0}, x, y, z, camxrot, camyrot, context, graphics);

        draw_line(&Point3D {x: cube_x + 50.0, y: cube_y + 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x + 50.0, y: cube_y + 50.0, z: cube_z + 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_line(&Point3D {x: cube_x - 50.0, y: cube_y + 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y + 50.0, z: cube_z + 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_line(&Point3D {x: cube_x - 50.0, y: cube_y - 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x - 50.0, y: cube_y - 50.0, z: cube_z + 50.0}, x, y, z, camxrot, camyrot, context, graphics);
        draw_line(&Point3D {x: cube_x + 50.0, y: cube_y - 50.0, z: cube_z - 50.0}, &Point3D {x: cube_x + 50.0, y: cube_y - 50.0, z: cube_z + 50.0}, x, y, z, camxrot, camyrot, context, graphics);
    }

    const WINDOW_X: i32 = 640;
    const WINDOW_Y: i32 = 480;
    const FOV: i32 = 400;
    const ZCLIPDIST: f64 = 3.0;

    const PI: f64 = std::f64::consts::PI;

    let mut window: PistonWindow = WindowSettings::new("Rust Craft", [WINDOW_X as u32, WINDOW_Y as u32])
        .exit_on_esc(true).build().unwrap();

    let mut x = 0.0;
    let mut y = 200.0;
    let mut z = 400.0;

    let mut camxrot: f64 = 0.0;
    let mut camyrot: f64 = 0.0;

    let mut pressed_keys = HashSet::new();

    //let mut rotation = 0.0;

    fn move_player(x: &mut f64, y: &mut f64, z: &mut f64, camxrot: f64, dir: i8) {
        if dir == 0 {
            let r = rotate_2d(&Point2D {x:0.0, y:5.0}, camxrot);
            *x += r.x;
            *z += r.y;
        } else if dir == 1 {
            let r = rotate_2d(&Point2D {x:5.0, y:0.0}, camxrot);
            *x += r.x;
            *z += r.y;
        } else if dir == 2 {
            let r = rotate_2d(&Point2D {x:0.0, y:-5.0}, camxrot);
            *x += r.x;
            *z += r.y;
        } else if dir == 3 {
            let r = rotate_2d(&Point2D {x:-5.0, y:0.0}, camxrot);
            *x += r.x;
            *z += r.y;
        }
    }    

    while let Some(event) = window.next() {
        if let Some(Button::Keyboard(key)) = event.press_args() {
            if let Some(Button::Keyboard(key)) = event.press_args() {
                pressed_keys.insert(key);
            }
    
            if let Some(Button::Keyboard(key)) = event.release_args() {
                pressed_keys.remove(&key);
            }

            for &key in pressed_keys.iter() {
                match key {
                    Key::Right => camxrot -= 0.05,
                    Key::Left => camxrot += 0.05,
                    Key::Up => camyrot -= 0.1,
                    Key::Down => camyrot += 0.1,
                    Key::W => move_player(&mut x, &mut y, &mut z, camxrot, 2),
                    Key::A => move_player(&mut x, &mut y, &mut z, camxrot, 1),
                    Key::S => move_player(&mut x, &mut y, &mut z, camxrot, 0),
                    Key::D => move_player(&mut x, &mut y, &mut z, camxrot, 3),
                    _ => {}
                }
            }

            camxrot = camxrot % (2.0 * PI);
            camyrot = camyrot.max(-1.5).min(1.5);

        } else {
            pressed_keys.clear();
        }

        window.draw_2d(&event, |context, graphics, _| {
            clear([1.0; 4], graphics);

            for c_x in -8..8 {
                for c_z in -8..8 {
                    cube(c_x as f64 * -100.0, 0.0, c_z as f64 * -100.0, x, y, z, camxrot, camyrot, context, graphics);
                }
            }

            //cube(0.0, 0.0, 0.0, x, y, z, camxrot, camyrot, context, graphics);
            //line([0.0, 0.0, 1.0, 1.0], 1.0, [1.0, 0.0, 0.0, 1.0], context.transform, graphics);
        });
    }
}