use crate::*;

use tubaitu::Cube2;

use shared::Move;
use geo::{coord, Coord};
use m_per_n::Vec3;

pub struct Scene {
    width: usize,
    height: usize,
    scale: f64,
    cube: DrawableCube2,
    camera: Camera,
    light_dir: Vec3,
}

impl Scene {
    // Given

    pub fn new(width: usize, height:usize, scale: f64, cube: Cube2, turn: Move, lerp_t: f64) -> Self {
        let cube = DrawableCube2::new(cube, turn, lerp_t);
        
        let cam_pos = Vec3::new(10.1, -30.1, 10.1) * 10.0;
        let cam_dir = Vec3::ZERO - cam_pos;
        let camera = Camera{
            position: cam_pos,
            direction: cam_dir.normalize().unwrap(),
            camera_plane_distance: cam_dir.abs(),
        };

        let light_dir = Vec3::ZERO - Vec3::new(10.1, -20.1, 30.1);
        
        Scene{
            width: width,
            height: height,
            scale: scale,
            cube: cube,
            camera: camera,
            light_dir: light_dir,
        }
    }

    pub fn draw(&mut self) -> Vec<Quadrilateral>{
        let quads = self.cube.get_drawing_data(&self.camera, self.light_dir);
        let mut result: Vec<Quadrilateral> = vec![];
        for quad in quads {
            let mut verts = [Coord::default();4];
            for (i, vert) in quad.vertices.iter().enumerate() {
                let x = (vert.x * self.scale + 0.5*self.width  as f64);
                let y = (vert.y * self.scale + 0.5*self.height as f64);
                verts[i] = coord!(x:x, y:y);
            }
            result.push( Quadrilateral {
                vertices: verts,
                color:quad.color,
            });
        }
        result
        /*
        for face in projected_cube {
        let mut polygon_points = vec![];
        for i in 0..4 {
            let x: usize = (face.vertices[i][0] * scale + 0.5 * width as f64) as usize;
            let y: usize = (face.vertices[i][1] * scale + 0.5 * height as f64) as usize;
            polygon_points.push((x, y));
        }

        buffer.push(Polygon {
            points: polygon_points,
            color: face.color.to_rgb(face.brightness),
        });
    } */
    }
}