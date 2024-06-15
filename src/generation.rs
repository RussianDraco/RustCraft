use crate::*;

pub const CHUNK_SIZE: f64 = 6.0;
const CHUNK_HEIGHT: f64 = 10.0;
const NORM_CHUNK_REND_DIST: i32 = CHUNK_RENDER_DIST as i32;
const PERLIN_SOFTENER: f64 = 15.0;

pub fn generate_cubes(pos: Triple, perlin_gen: Perlin) -> Vec<Cube> {
    let chunk_x = (pos.x as f64 / CHUNK_SIZE).floor() as i32;
    let chunk_y = (pos.z as f64 / CHUNK_SIZE).floor() as i32;

    let mut cubes: Vec<Cube> = Vec::new();

    for _chunk_x in chunk_x-NORM_CHUNK_REND_DIST..chunk_x+NORM_CHUNK_REND_DIST+1 {
        for _chunk_y in chunk_y-NORM_CHUNK_REND_DIST..chunk_y+NORM_CHUNK_REND_DIST+1 {
            let chunk = load_chunk(_chunk_x, _chunk_y, perlin_gen);
            
            let mut _cx = 0;
            for cx in chunk {
                let mut _cz = 0;
                for cz in cx {
                    cubes.push(init_cube(Triple {
                        x: _cx as f64 + _chunk_x as f64 * CHUNK_SIZE,
                        y: cz as f64,
                        z: _cz as f64 + _chunk_y as f64 * CHUNK_SIZE,
                    }));
                    
                    _cz += 1;
                }
                _cx += 1;
            }
        }
    }
    cubes
}

fn load_chunk(chunk_x: i32, chunk_y: i32, perlin_gen: Perlin) -> Vec<Vec<u8>> { //use seed
    let mut chunk: Vec<Vec<u8>> = Vec::new();
    for x in 0..CHUNK_SIZE as usize {
        let mut row: Vec<u8> = Vec::new();
        for z in 0..CHUNK_SIZE as usize {
            row.push((perlin_gen.get([(chunk_x as f64 + x as f64) / PERLIN_SOFTENER, (chunk_y as f64 + z as f64) / PERLIN_SOFTENER]) * CHUNK_HEIGHT).round() as u8);
        }
        chunk.push(row);
    }
    chunk
}