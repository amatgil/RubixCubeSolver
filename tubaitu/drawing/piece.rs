use geo::{Polygon, LineString, Coord, BooleanOps, CoordsIter, Centroid};
use drawing::{Vertex, Stiker, Ray};
use m_per_n::Vec3;

struct DrawablePiece {
    vertices: [Vertex; 8],
    faces: [Stiker; 6],
    drawing_order: [u8; 6],
}

impl DrawablePiece {
    fn new(vertices: [Vec3;8], rotation:PieceRotation) -> Self{
        todo!();
    }

    fn apply_rotation(axis: u8, clockwhise: bool) {
        todo!();
    }

    fn project_vertices(camera: Camera, light_dir: Vec3) {
        todo!();
    }

    fn update_stikers() {
        todo!();
    }

    fn find_displaying_order(camera: Camera) {
        todo!();
    }

    fn get_outline_polygon() -> Polygon {
        todo!();
    }

    fn get_overlap_centroid_2d(piece: DrawablePiece, camera: Camera) -> Option<Coord> {
        todo!();
    }

    fn get_intersections_with_ray(ray: Ray) -> Vec<Vec3> {
        todo!();
    }

    fn is_in_front(piece: DrawablePiece, camera: Camera) -> Option<Ordering> {
        todo!();
    }

    fn get_drawing_data() -> Vec<Polygon> {
        todo!();
    }
}