use m_per_n::Vec3;

pub struct Vertex {
    pub _3d: Vec3,
    pub _2d: Coord,
}

struct Stiker {
    vertices: [Vertex; 4],
    normal_vec: Vec3,
    color: Color,
    brightness: f64,
}

impl Stiker {
    fn new(vertices_3d: [Vertex; 4], color: Color) -> Self {
        todo!();
    }

    fn update_brightness(light_dir: Vec3) {
        todo!();
    }

    fn get_polygon() -> Polygon {
        todo!();
    }

    fn get_overlap_centroid_2d(quad: Stiker, camera: Camera) -> Option<Coord> {
        todo!();
    }

    fn get_drawing_data() {
        todo!();
    }
}