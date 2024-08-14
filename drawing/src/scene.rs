use crate::*;

use tubaitu::Cube2;

use shared::Move;

use m_per_n::Vec3;

pub struct Scene {
    cube: DrawableCube2,
    camera: Camera,
    light_dir: Vec3,
}

impl Scene {
    // Given

    pub fn new(cube: Cube2, turn: Move, lerp_t: f64) -> Self {
        let cube = DrawableCube2::new(cube, turn, lerp_t);
        
        let cam_pos = Vec3::new(10.1, -30.1, 10.1) * 10.0;
        let cam_dir = Vec3::ZERO - cam_pos;
        
        let camera = Camera{
            position: cam_pos,
            direction: cam_dir,
            camera_plane_distance: DEFAULT_CAMERA_PLANE_DISTANCE,
        };

        let light_dir = Vec3::ZERO - Vec3::new(10.1, -20.1, 30.1);
        
        Scene{
            cube: cube,
            camera: camera,
            light_dir: light_dir,
        }
    }

    pub fn draw(&self) -> Vec<Quadrilateral>{
        todo!();
    }
}


#[test]
fn test() {
}