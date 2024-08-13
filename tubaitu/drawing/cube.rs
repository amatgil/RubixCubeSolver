use drawing::DrawablePiece;
use m_per_n::Vec3;

struct DrawableCube2 {
    pieces: [DrawablePiece; 8],
    drawing_order: [u8; 8],
}

impl DrawableCube2 {
    fn new(cube: Cube2, mov: Move, lerp_t: f64) -> Self{
        todo!();
    }

    fn find_displaying_order(camera: Camera) {
        todo!();
    }

    fn get_drawing_data(camera: Camera, light_dir: Vec3) -> Vec<Quadrilateral> {
        todo!();
    }
}