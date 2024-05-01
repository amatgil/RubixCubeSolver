use crate::*;
use m_per_n::*;

use m_per_n::Vec3;
use std::cmp::Ordering;
use std::f64::consts::PI;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf}; // The lesser circle constant

const WIDTH: usize = 10000;
const HEIGHT: usize = 10000;
const MIN_BRIGHTNESS_MULTIPLIER: f64 = 0.5;
const GENERAL_BRIGHTNESS_MULTIPLIER: f64 = 1.0;
const DISTANCE_CAMERA_PLANE: f64 = 1.0;

#[derive(Clone, Debug, Copy)]
struct Camera {
    pos: Vec3,
    dir: Vec3,
}

#[derive(Clone, Copy, Default, Debug)]
struct DrawablePiece {
    rotation: PieceRotation,
    center: Point,
    radius: f64,
    should_rotate: bool,
}

#[derive(Copy, Clone)]
struct Quadrilateral {
    distance: f64,
    vertices: Matrix<4, 2>,
    brightness: f64,
    color: Color,
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
    fn empty() -> Self {
        Quadrilateral {
            distance: 0.0,
            vertices: Matrix::ZERO(),
            brightness: 0.0,
            color: Color::Blue,
        }
    }
}

fn furthest_vertex_from_point(vertices: [Vec3; 4], point: Vec3) -> f64 {
    *vertices
        .map(|x| (x - point).abs())
        .iter()
        .max_by(|a, b| a.total_cmp(b))
        .unwrap()
}

fn closest_vertex_to_point(vertices: [Vec3; 4], point: Vec3) -> f64 {
    *vertices
        .map(|x| (x - point).abs())
        .iter()
        .min_by(|a, b| a.total_cmp(b))
        .unwrap()
}

fn get_rotation_matrix(mov: Move, mut lerp_t: f64) -> Matrix<3, 3> {
    if mov.side() == MoveSide::L || mov.side() == MoveSide::D || mov.side() == MoveSide::B {
        lerp_t *= -1.0;
    }

    lerp_t *= if mov.is_prime() { -1.0 } else { 1.0 };

    let cos = (lerp_t * PI / 2.0).cos();
    let sin = (lerp_t * PI / 2.0).sin();
    let matrix: Matrix<3, 3> = match mov.side() {
        MoveSide::R | MoveSide::L => Matrix::<3, 3>([
            MatRow::<3>([1.0, 0.0, 0.0]),
            MatRow::<3>([0.0, cos, sin]),
            MatRow::<3>([0.0, -sin, cos]),
        ]),

        MoveSide::U | MoveSide::D => Matrix::<3, 3>([
            MatRow::<3>([cos, sin, 0.0]),
            MatRow::<3>([-sin, cos, 0.0]),
            MatRow::<3>([0.0, 0.0, 1.0]),
        ]),

        MoveSide::F | MoveSide::B => Matrix::<3, 3>([
            MatRow::<3>([cos, 0.0, sin]),
            MatRow::<3>([0.0, 1.0, 0.0]),
            MatRow::<3>([-sin, 0.0, cos]),
        ]),
    };
    matrix
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
            vertices[i] = match i {
                P_TOP_RIGHT_FRONT => vertices[0] + Vec3::new(r, -r, r),
                P_TOP_RIGHT_BACK => vertices[1] + Vec3::new(r, r, r),
                P_TOP_LEFT_BACK => vertices[2] + Vec3::new(-r, r, r),
                P_TOP_LEFT_FRONT => vertices[3] + Vec3::new(-r, -r, r),
                P_BOTTOM_RIGHT_FRONT => vertices[4] + Vec3::new(r, -r, -r),
                P_BOTTOM_RIGHT_BACK => vertices[5] + Vec3::new(r, r, -r),
                P_BOTTOM_LEFT_BACK => vertices[6] + Vec3::new(-r, r, -r),
                P_BOTTOM_LEFT_FRONT => vertices[7] + Vec3::new(-r, -r, -r),
                _ => unreachable!("Vertex index not valid?"),
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
        let root3: f64 = 3.0f64.sqrt();

        // faces are the
        let mut projected_faces: [Quadrilateral; 6] = [Quadrilateral::empty(); 6];
        let mut faces: [[Vec3; 4]; 6] = [[Vec3::ZERO; 4]; 6];

        faces[SIDE_RIGHT] = [verts[0], verts[1], verts[5], verts[4]];
        faces[SIDE_FRONT] = [verts[0], verts[3], verts[7], verts[4]];
        faces[SIDE_TOP] = [verts[0], verts[1], verts[2], verts[3]];
        faces[SIDE_LEFT] = [verts[2], verts[3], verts[7], verts[6]];
        faces[SIDE_BACK] = [verts[1], verts[2], verts[6], verts[5]];
        faces[SIDE_DOWN] = [verts[4], verts[5], verts[6], verts[7]];

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
            let vec = vertices[i] - camera.pos;
            let intersection_option = Self::find_intersection(vertices[i], camera);
            intersections[i] = match intersection_option {
                Some(x) => x,
                None => Vec3::new(0.0, 0.0, 0.0),
            };
        }
        Self::to_xy_plane(intersections, camera)
    }

    fn draw(&self, camera: Camera, light_dir: Vec3, mov: Move, lerp_t: f64) -> [Quadrilateral; 6] {
        let vertices = self.get_vertex_positions(mov, lerp_t);
        let projected_vertices = Self::project_points(vertices, camera);

        let mut projected_faces =
            self.get_polygons_with_brightness(light_dir, camera.pos, vertices, projected_vertices);
        // Sorts the polygons from furthest to nearest.
        projected_faces.sort_by(|a, b| b.cmp(a));

        projected_faces
    }
}

fn get_normal_vector(face: [Vec3; 4], center: Vec3) -> Vec3 {
    let normal = (face[1] - face[0])
        .cross_product(face[2] - face[0])
        .normalize()
        .unwrap();
    let dot_product = normal.dot_product(center - face[0]);

    normal * if dot_product < 0.0 { -1.0 } else { 1.0 }
}

#[derive(Clone, Debug, Default, Copy)]
struct DrawableCube {
    pieces: [DrawablePiece; 8],
}

const DRAWING_PIECE_RADIUS: f64 = 10.0;
const EXTRA_PIECE_DISTANCE: f64 = 0.8;
impl Cube2 {
    fn to_points(self) -> DrawableCube {
        let r = DRAWING_PIECE_RADIUS + EXTRA_PIECE_DISTANCE;
        let mut drawable_pieces = [DrawablePiece::default(); 8];

        for (piece_idx, original_piece) in self.pieces.iter().enumerate() {
            let rotation: PieceRotation = original_piece.rotation;
            let center: Point = match piece_idx {
                P_TOP_RIGHT_FRONT => Point::new(r, -r, r),
                P_TOP_RIGHT_BACK => Point::new(r, r, r),
                P_TOP_LEFT_BACK => Point::new(-r, r, r),
                P_TOP_LEFT_FRONT => Point::new(-r, -r, r),
                P_BOTTOM_RIGHT_FRONT => Point::new(r, -r, -r),
                P_BOTTOM_RIGHT_BACK => Point::new(r, r, -r),
                P_BOTTOM_LEFT_BACK => Point::new(-r, r, -r),
                P_BOTTOM_LEFT_FRONT => Point::new(-r, -r, -r),
                _ => unreachable!("Piece index no vàlid?"),
            };

            drawable_pieces[piece_idx] = DrawablePiece {
                center,
                radius: DRAWING_PIECE_RADIUS,
                rotation,
                should_rotate: false,
            };
        }

        DrawableCube {
            pieces: drawable_pieces,
        }
    }
}

/// Draw the solving sequence, with `n_in_between_frames`*`moves.len()` frames
pub fn draw_sequence(
    file_prefix: &Path,
    starting_cube: &Cube2,
    moves: MoveSeq,
    n_in_between_frames: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cube: Cube2 = *starting_cube;

    for (i, mov) in moves.iter().enumerate() {
        let i = i * n_in_between_frames;
        for inbetween_index in 0..n_in_between_frames {
            let lerp_t = inbetween_index as f64 / n_in_between_frames as f64;
            let mut filename_str = file_prefix.to_str().unwrap().to_owned();
            filename_str.push_str(&format!("_{:>04}", i + inbetween_index));
            println!("Generating: {:?}", filename_str);

            let svg: String = get_svg(cube, *mov, lerp_t);

            let mut file: fs::File = fs::File::create::<PathBuf>(filename_str.into())?;
            file.write_all(svg.as_bytes())?;
        }
        cube.make_move(*mov);
    }

    Result::<(), Box<dyn std::error::Error>>::Ok(())
}

/// Given a cube, the move being done and how far along the move is, generate the corresponding svg as a String. This is a self-contained frame representing the cube in the given state.
fn get_svg(cube: Cube2, mov: Move, lerp_t: f64) -> String {
    let mut pieces = cube.to_points().pieces; // Un array de 8 DrawablePieces, que contenen els seus punts
                                              // Recorda que el radi és DRAWING_PIECE_RADIUS
    format!("{cube} with {mov:?} at with lerp value {lerp_t}");

    let light_pos = Vec3::new(10.0, -20.0, 30.0);
    let light_dir = Vec3::ZERO - light_pos;

    let pos = Vec3::new(10.0, -30.0, 10.0) * 10.0;

    let camera: Camera = Camera {
        pos,
        dir: Vec3::ZERO - pos,
    };

    let pieces_to_cycle = match mov.side() {
        MoveSide::R => FACE_RIGHT_SEQ_CYCLE,
        MoveSide::L => FACE_LEFT_SEQ_CYCLE,
        MoveSide::U => FACE_UP_SEQ_CYCLE,
        MoveSide::D => FACE_DOWN_SEQ_CYCLE,
        MoveSide::F => FACE_FRONT_SEQ_CYCLE,
        MoveSide::B => FACE_BACK_SEQ_CYCLE,
    };

    for i in pieces_to_cycle {
        pieces[i].should_rotate = true;
    }

    let mut projected_cube: [Quadrilateral; 48] = [Quadrilateral::empty(); 48];

    for (i, piece) in pieces.iter().enumerate() {
        let aux = piece.draw(camera, light_dir, mov, lerp_t);
        for j in 0..6 {
            projected_cube[6 * i + j] = aux[j];
        }
    }

    projected_cube.sort_by(|a, b| a.cmp(b).reverse());

    let mut buffer = String::new();

    buffer.push_str(&format!("<svg viewBox=\"0 0 {WIDTH} {HEIGHT} \" style=\"background-color:{BACKGROUND_COL}\" xmlns=\"http://www.w3.org/2000/svg\" id=\"rubix-cube\">\n"));

    for face in projected_cube {
        buffer.push_str("<polygon points=\"");
        for i in 0..4 {
            let x: usize = (face.vertices[i][0] * 100.0 + 0.5 * WIDTH as f64) as usize;
            let y: usize = (face.vertices[i][1] * 100.0 + 0.5 * HEIGHT as f64) as usize;
            buffer.push_str(&format!("{x},{y} "));
        }

        let color: [usize; 3] = face.color.to_rgb(face.brightness);
        buffer.push_str(&format!(
            "\" fill=\"#{:02x}{:02x}{:02x}\" stroke=\"none\"/>\n",
            color[0], color[1], color[2]
        ));
    }

    buffer.push_str("</svg>\n");

    buffer
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

    let buffer = piece.draw(camera, light_dir, Move::R, 0.3);
    //println!("{}", buffer);
}

#[test]
fn test_drawing_cube() {
    let cube = Cube2::default();
    let m = Move::R;

    let text = get_svg(cube, m, 0.3);
    println!("{}", text);
}

#[test]

fn test_video() {
    let starting_cube = Cube2::default();
    let moves = [
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
    //draw_sequence("r_move_test", &starting_cube, &moves, 10);
}
