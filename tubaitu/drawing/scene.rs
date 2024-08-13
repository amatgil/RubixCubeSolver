use drawing::DrawableCube2;
use drawing::Camera;
use drawing::Quadrilateral;

use tubaitu::Cube2;

use shared::Move;

use m_per_n::Vec3;

struct Scene {
    cube: DrawableCube2,
    camera: Camera,
    light_dir: Vec3,
}

impl Scene {
    // Given
    fn new(cube: Cube2, turn: Move, lerp_t: f64) -> Self {
        todo!();
    }

    fn draw() -> Vec<Quadrilateral>{
        todo!();
    }
}
