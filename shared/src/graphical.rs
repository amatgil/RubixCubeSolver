use crate::*;
use m_per_n::*;


pub const MIN_BRIGHTNESS_MULTIPLIER: f64 = 0.5;
pub const GENERAL_BRIGHTNESS_MULTIPLIER: f64 = 1.0;
pub const DISTANCE_CAMERA_PLANE: f64 = 1.0;

pub const DRAWING_PIECE_RADIUS: f64 = 10.0;
pub const EXTRA_PIECE_DISTANCE: f64 = 0.8;

pub trait Drawable<const PS: usize> {
    type DrawablePiece; // 
    fn to_points(self) -> [Self::DrawablePiece; PS];
}

#[derive(Clone, Debug, Copy)]
pub struct Camera {
    pub pos: Vec3,
    pub dir: Vec3,
}

#[derive(Copy, Clone)]
pub struct Quadrilateral {
    pub distance: f64,
    pub vertices: Matrix<4, 2>,
    pub brightness: f64,
    pub color: Color,
}


// ======= UTILITY FUNCTIONS ========
pub fn furthest_vertex_from_point(vertices: [Vec3; 4], point: Vec3) -> f64 {
    *vertices
        .map(|x| (x - point).abs())
        .iter()
        .max_by(|a, b| a.total_cmp(b))
        .unwrap()
}

pub fn closest_vertex_to_point(vertices: [Vec3; 4], point: Vec3) -> f64 {
    *vertices
        .map(|x| (x - point).abs())
        .iter()
        .min_by(|a, b| a.total_cmp(b))
        .unwrap()
}

pub fn get_rotation_matrix(mov: Move, mut lerp_t: f64) -> Matrix<3, 3> {
    if mov.side() == MoveSide::L || mov.side() == MoveSide::D || mov.side() == MoveSide::B {
        lerp_t *= -1.0;
    }

    lerp_t *= if mov.is_prime() { -1.0 } else { 1.0 };

    let cos = (lerp_t * PI / 2.0).cos();
    let sin = (lerp_t * PI / 2.0).sin();

    match mov.side() {
        MoveSide::R | MoveSide::L => Matrix::<3, 3>([
            MatRow::<3>([1.0, 0.0, 0.0]),
            MatRow::<3>([0.0, cos, sin]),
            MatRow::<3>([0.0, -sin, cos]),
        ]),
        MoveSide::F | MoveSide::B => Matrix::<3, 3>([
            MatRow::<3>([cos, 0.0, sin]),

            MatRow::<3>([-sin, 0.0, cos]),

        ]),
        MoveSide::U | MoveSide::D => Matrix::<3, 3>([
            MatRow::<3>([cos, sin, 0.0]),
            MatRow::<3>([-sin, cos, 0.0]),
            MatRow::<3>([0.0, 0.0, 1.0]),
        ]),

    }
}


pub fn get_normal_vector(face: [Vec3; 4], center: Vec3) -> Vec3 {
    let normal = (face[1] - face[0])
        .cross_product(face[2] - face[0])
        .normalize()
        .unwrap();
    let dot_product = normal.dot_product(center - face[0]);

    normal * if dot_product < 0.0 { -1.0 } else { 1.0 }
}


impl PartialEq for Quadrilateral {
    fn eq(&self, other: &Self) -> bool {
        (self.brightness - other.brightness).abs() < FLOAT_EPSILON
            && (self.distance - other.distance).abs() < FLOAT_EPSILON
            && self.vertices == other.vertices
    }
}

impl Eq for Quadrilateral {}

impl PartialOrd for Quadrilateral {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Quadrilateral {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance
            .partial_cmp(&other.distance)
            .expect("Tried to order pieces when one of the distances was NaN")
    }
}

impl Quadrilateral {
    pub fn empty() -> Self {
        Quadrilateral {
            distance: 0.0,
            vertices: Matrix::ZERO(),
            brightness: 0.0,
            color: Color::Blue,
        }
    }
}
