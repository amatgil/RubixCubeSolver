
pub struct Vertex {
    pub _3d: Vec3,
    pub _2d: Coord,
}

struct Stiker {
    vertices: [Vertex; 4],
    normmal_vec: Vec3,
    color: Color,
    brightness: f64,
}