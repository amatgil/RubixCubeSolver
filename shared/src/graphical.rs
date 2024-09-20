use crate::*;
use m_per_n::*;

mod piece;
mod scene;
mod sticker;
mod utils;

use piece::*;
use scene::*;
use sticker::*;
use utils::*;

pub const MIN_BRIGHTNESS_MULTIPLIER: f64 = 0.5;
pub const GENERAL_BRIGHTNESS_MULTIPLIER: f64 = 1.0;
pub const DISTANCE_CAMERA_PLANE: f64 = 1.0;

pub const DRAWING_PIECE_RADIUS: f64 = 10.0;
pub const EXTRA_PIECE_DISTANCE: f64 = 0.65;

const PIECE_RADIUS: f64 = 10.0;
const DEFAULT_CAMERA_PLANE_DISTANCE: f64 = 1.0;

pub trait Drawable<const PS: usize> {
    fn to_points(self) -> [DrawablePiece; PS];
    /// Given a cube, the move being done and how far along the move is, generate the corresponding polys that would draw it
    fn get_polys(&self, part_mov: Option<PartialMove>, width: usize, height: usize, scale: f64) -> Vec<Polygon>;
}


pub struct Polygon {
    pub points: Vec<(usize, usize)>,
    pub color: [usize; 3],
}

#[derive(Debug, Clone)]
pub struct PartialMove {
    pub mov: Move,
    pub lerp_t: f64,
}
#[repr(usize)]
#[derive(Debug, Clone, Copy)]
enum VertexPosition {
    TopRightFront,
    TopRightBack,
    TopLeftBack,
    TopLeftFront,
    BottomRightFront,
    BottomRightBack,
    BottomLeftBack,
    BottomLeftFront,
}
type V = VertexPosition;

const FACE_RIGHT_SEQ_CYCLE: [V; 4] = [V::TopRightBack,    V::BottomRightBack, V::BottomRightFront, V::TopRightFront];
const FACE_LEFT_SEQ_CYCLE: [V; 4]  = [V::TopLeftFront,    V::BottomLeftFront, V::BottomLeftBack,   V::TopLeftBack];
const FACE_UP_SEQ_CYCLE: [V; 4]    = [V::TopLeftFront,    V::TopLeftBack,     V::TopRightBack,     V::TopRightFront];
const FACE_DOWN_SEQ_CYCLE: [V; 4]  = [V::BottomLeftBack,  V::BottomLeftFront, V::BottomRightFront, V::BottomRightBack];
const FACE_FRONT_SEQ_CYCLE: [V; 4] = [V::BottomLeftFront, V::TopLeftFront,    V::TopRightFront,    V::BottomRightFront];
const FACE_BACK_SEQ_CYCLE: [V; 4]  = [V::BottomLeftBack,  V::BottomRightBack, V::TopRightBack,     V::TopLeftBack];

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
            MatRow::<3>([0.0,  1.0, 0.0]),
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


#[derive(Clone, Copy, Default, Debug)]
pub struct DrawablePiece {
    pub rotation: PieceRotation,
    pub center: Point,
    pub radius: f64,
    pub should_rotate: bool,
}

impl DrawablePiece {
    /// Returns an array of row matricies that correspond to the positions of the
    /// pieces' vertices.
    fn get_vertex_positions(&self, mov: Move, lerp_t: f64) -> [Vec3; 8] {
        let r = self.radius;
        let c = self.center;
        let mut vertices = [Vec3::new(c.x, c.y, c.z); 8];
        let mut transformation: Matrix<3, 3> = Matrix::<3, 3>::ID();
        if self.should_rotate {
            transformation = get_rotation_matrix(mov, lerp_t);
        }
        for i in 0..8 {
            vertices[i] = match i.try_into().unwrap() {
                VertexPosition::TopRightFront    => vertices[0] + Vec3::new(r, -r, r),
                VertexPosition::TopRightBack     => vertices[1] + Vec3::new(r, r, r),
                VertexPosition::TopLeftBack      => vertices[2] + Vec3::new(-r, r, r),
                VertexPosition::TopLeftFront     => vertices[3] + Vec3::new(-r, -r, r),
                VertexPosition::BottomRightFront => vertices[4] + Vec3::new(r, -r, -r),
                VertexPosition::BottomRightBack  => vertices[5] + Vec3::new(r, r, -r),
                VertexPosition::BottomLeftBack   => vertices[6] + Vec3::new(-r, r, -r),
                VertexPosition::BottomLeftFront  => vertices[7] + Vec3::new(-r, -r, -r),
            };
            if self.should_rotate {
                let col_vec: Matrix<3, 1> = vertices[i].into(); // Write as column vector
                vertices[i] = (transformation * col_vec).into();
            }
        }
        vertices
    }

    // Necessito una funció que em doni: un vector de:
    //      - cares projectades
    //      - amb un index que les ordeni segons la distància a la càmera
    //      - la brillantor que ha de tenir la cara.

    /// Pre: vertices and projected_vertices are ordered as shown in README file.
    /// Returns an array of 6 tuples, containing the following information about each of the faces:
    ///
    /// - The distance of the furthest point from the cammera
    /// - A 4x2 matrix with the coordiantes of its vertices (Projected into the XY plane)
    /// - Brightness level ranging from 0 to 1
    fn get_polygons_with_brightness(
        &self,
        light_dir: Vec3,
        camera_pos: Vec3,
        verts: [Vec3; 8],
        projected_verts: Matrix<8, 2>,
    ) -> [Quadrilateral; 6] {
        let root3: f64 = 3.0f64.sqrt() + 0.1;

        // faces are the
        let mut projected_faces: [Quadrilateral; 6] = [Quadrilateral::empty(); 6];
        let mut faces: [[Vec3; 4]; 6] = [[Vec3::ZERO; 4]; 6];

        faces[*Side::Right] = [
            verts[*V::TopRightFront],    verts[*V::TopRightBack],     verts[*V::BottomRightBack], verts[*V::BottomRightFront]
        ];
        faces[*Side::Front] = [
            verts[*V::TopRightFront],    verts[*V::BottomRightFront], verts[*V::BottomLeftFront], verts[*V::TopLeftFront]
        ];
        faces[*Side::Top] =  [
            verts[*V::TopRightFront],    verts[*V::TopLeftFront],     verts[*V::TopLeftBack],     verts[*V::TopRightBack]
        ];
        faces[*Side::Left] = [
            verts[*V::TopLeftFront],     verts[*V::BottomLeftFront],  verts[*V::BottomLeftBack],  verts[*V::TopLeftBack]
        ];
        faces[*Side::Back] = [
            verts[*V::TopRightBack],     verts[*V::TopLeftBack],      verts[*V::BottomLeftBack],  verts[*V::BottomRightBack], 
            ];
        faces[*Side::Down] = [
            verts[*V::BottomRightFront], verts[*V::BottomRightBack],  verts[*V::BottomLeftBack],  verts[*V::BottomLeftFront]
        ];

        projected_faces[0].vertices = Matrix::<4, 2>([
            projected_verts[0],
            projected_verts[1],
            projected_verts[5],
            projected_verts[4],
        ]);
        projected_faces[1].vertices = Matrix::<4, 2>([
            projected_verts[0],
            projected_verts[3],
            projected_verts[7],
            projected_verts[4],
        ]);
        projected_faces[2].vertices = Matrix::<4, 2>([
            projected_verts[0],
            projected_verts[1],
            projected_verts[2],
            projected_verts[3],
        ]);
        projected_faces[3].vertices = Matrix::<4, 2>([
            projected_verts[2],
            projected_verts[3],
            projected_verts[7],
            projected_verts[6],
        ]);
        projected_faces[4].vertices = Matrix::<4, 2>([
            projected_verts[1],
            projected_verts[2],
            projected_verts[6],
            projected_verts[5],
        ]);
        projected_faces[5].vertices = Matrix::<4, 2>([
            projected_verts[4],
            projected_verts[5],
            projected_verts[6],
            projected_verts[7],
        ]);

        let colors = (Piece {
            rotation: self.rotation,
        })
        .to_color_sequence();

        let center = Vec3::new(self.center.x, self.center.y, self.center.z);
        for i in 0..6 {
            projected_faces[i].distance = furthest_vertex_from_point(faces[i], camera_pos);

            let normal_vector = get_normal_vector(faces[i], center);
            let dot_product = normal_vector.dot_product(light_dir.normalize().unwrap());
            projected_faces[i].brightness =
                MIN_BRIGHTNESS_MULTIPLIER.max(dot_product * GENERAL_BRIGHTNESS_MULTIPLIER);
            projected_faces[i].color = colors[i];

            let dist_to_origin = closest_vertex_to_point(faces[i], Vec3::ZERO);
            if dist_to_origin < root3 * EXTRA_PIECE_DISTANCE {
                projected_faces[i].color = Color::White;
                projected_faces[i].brightness = 0.0;
            }
        }
        projected_faces
    }

    fn find_intersection(point: Vec3, camera: Camera) -> Option<Vec3> {
        let v = point - camera.pos;
        let n = camera.dir;
        let p1 = point;
        let p2 = camera.pos + camera.dir * DISTANCE_CAMERA_PLANE;

        // System of equations to find intersection point.
        let mut eq: Matrix<3, 4> = Matrix::<3, 4>::ZERO();
        eq[0] = MatRow::<4>([v.y, -v.x, 0.0, p1.x * v.y - p1.y * v.x]);
        eq[1] = MatRow::<4>([0.0, -v.z, v.y, p1.z * v.y - p1.y * v.z]);
        eq[2] = MatRow::<4>([n.x, n.y, n.z, p2.x * n.x + p2.y * n.y + p2.z * n.z]);

        // Gaussian elimination (Don't look at this please. I just copy pasted from my old code '>_>)
        eq[1] = eq[0][0] * eq[1] - eq[1][0] * eq[0];
        eq[2] = eq[0][0] * eq[2] - eq[2][0] * eq[0];
        eq[2] = eq[1][1] * eq[2] - eq[2][1] * eq[1];

        let mut result: Vec3 = Vec3::ZERO;
        if eq[0][0] == 0.0 || eq[1][1] == 0.0 || eq[2][2] == 0.0 {
            return None;
        }
        result.z = eq[2][3] / eq[2][2];
        result.y = (eq[1][3] - eq[1][2] * result.z) / eq[1][1];
        result.x = (eq[0][3] - eq[0][2] * result.z - eq[0][1] * result.y) / eq[0][0];

        Some(result)
    }

    fn to_xy_plane(vertices: [Vec3; 8], camera: Camera) -> Matrix<8, 2> {
        let n = camera.dir;
        let basis1 = Vec3::new(
            n.y * 0.0 - n.z * n.y,
            n.z * n.x - n.x * 0.0,
            n.x * n.y - n.y * n.x,
        );
        let n_i = basis1.normalize().unwrap();

        let basis2 = Vec3::new(
            n.y * basis1.z - n.z * basis1.y,
            n.z * basis1.x - n.x * basis1.z,
            n.x * basis1.y - n.y * basis1.x,
        );
        let n_j = basis2.normalize().unwrap();

        let transformation = Matrix::<3, 3>([
            MatRow::<3>([n_i.x, n_j.x, n.x]),
            MatRow::<3>([n_i.y, n_j.y, n.y]),
            MatRow::<3>([n_i.z, n_j.z, n.z]),
        ]);

        let inverse_transformation: Matrix<3, 3> = transformation.inverse().unwrap();
        let mut result = Matrix::<8, 2>::ZERO();

        let aux = camera.pos + camera.dir * DISTANCE_CAMERA_PLANE;
        let cam_projection = MatRow::<3>([aux.x, aux.y, aux.z]);

        for (i, v) in vertices.iter().enumerate() {
            let column_input =
                (Matrix::<1, 3>([MatRow::<3>([v.x, v.y, v.z]) - cam_projection])).transpose();
            let column = inverse_transformation * column_input;
            result[i] = MatRow::<2>([column[0][0], column[1][0]]);
        }
        result
    }

    fn project_points(vertices: [Vec3; 8], camera: Camera) -> Matrix<8, 2> {
        let mut intersections: [Vec3; 8] = [Vec3::ZERO; 8];
        for i in 0..8 {
            let intersection_option = Self::find_intersection(vertices[i], camera);
            intersections[i] = match intersection_option {
                Some(x) => x,
                None => Vec3::new(0.0, 0.0, 0.0),
            };
        }
        Self::to_xy_plane(intersections, camera)
    }

    pub fn draw(&self, camera: Camera, light_dir: Vec3, mov: Move, lerp_t: f64) -> [Quadrilateral; 6] {
        let vertices = self.get_vertex_positions(mov, lerp_t);
        let projected_vertices = Self::project_points(vertices, camera);

        let mut projected_faces =
            self.get_polygons_with_brightness(light_dir, camera.pos, vertices, projected_vertices);
        // Sorts the polygons from furthest to nearest.
        projected_faces.sort_by(|a, b| b.cmp(a));

        projected_faces
    }
}

#[test]
fn test_drawing_piece() {
    let piece = DrawablePiece {
        rotation: PieceRotation::WB,
        center: Point {
            x: 5.0,
            y: 5.0,
            z: 5.0,
        },
        radius: 5.0,
        should_rotate: false,
    };

    let pos = Vec3::new(20.0, 20.0, 20.0) * 10.0;

    let camera: Camera = Camera {
        pos,
        dir: Vec3::ZERO - pos,
    };

    let light_pos = Vec3::new(12.0, 20.2, 30.7);
    let light_dir = Vec3::ZERO - light_pos;

    let _buffer = piece.draw(camera, light_dir, Move::R, 0.3);
    //println!("{}", _buffer);
}


#[test]

fn test_video() {
    let _starting_cube = Cube2::default();
    let _moves = [
        Move::R,
        Move::U,
        Move::RP,
        Move::UP,
        Move::RP,
        Move::F,
        Move::R,
        Move::R,
        Move::UP,
        Move::RP,
        Move::UP,
        Move::R,
        Move::U,
        Move::RP,
        Move::FP,
    ];
    //draw_sequence("r_move_test", &_starting_cube, &_moves, 10);
}


impl TryFrom<usize> for VertexPosition {
    type Error = ();
    fn try_from(value: usize) -> Result<Self, ()> {
        if value > 7 { return Err(()) }
        else { Ok(unsafe { std::mem::transmute(value) })}
    }

}
impl Deref for VertexPosition {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute(self) }
    }
}

fn get_corner_cycle(mov: Move) -> [VertexPosition; 4] {
    match mov {
        Move::R => FACE_RIGHT_SEQ_CYCLE,
        Move::L => FACE_LEFT_SEQ_CYCLE,
        Move::U => FACE_UP_SEQ_CYCLE,
        Move::D => FACE_DOWN_SEQ_CYCLE,
        Move::F => FACE_FRONT_SEQ_CYCLE,
        Move::B => FACE_BACK_SEQ_CYCLE,
        Move::RP => FACE_RIGHT_SEQ_CYCLE,
        Move::LP => FACE_LEFT_SEQ_CYCLE,
        Move::UP => FACE_UP_SEQ_CYCLE,
        Move::DP => FACE_DOWN_SEQ_CYCLE,
        Move::FP => FACE_FRONT_SEQ_CYCLE,
        Move::BP => FACE_BACK_SEQ_CYCLE,
        _ => [0; 4],
    }
}
