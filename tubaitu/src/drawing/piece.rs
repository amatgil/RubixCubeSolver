
struct DrawablePiece {
    vertices: [Vertex; 8],
    faces: [Stiker; 6],
    drawing_order: [u8; 6],
}