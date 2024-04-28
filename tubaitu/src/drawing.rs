use m_per_n::*;
use crate::*;

use std::fs;
use std::io::Write;
use std::cmp::Ordering;
use m_per_n::Vec3;

const WIDTH :usize = 10000;
const HEIGHT:usize = 10000;
const MIN_BRIGHTNESS_MULTIPLIER: f64 = 0.1;
const GENERAL_BRIGHTNESS_MULTIPLIER: f64 = 0.8;
const DISTANCE_CAMERA_PLANE: f64 = 1.0;



/// A Piece with attached {center, radius} information for drawing
#[derive(Clone, Debug, Default, Copy)]
struct DrawablePiece {
    rotation: PieceRotation,
    center: Point,
    radius: f64,
}

#[derive(Copy, Clone, Default)]
struct OrderedPiece {
    distance: f64,
    piece: DrawablePiece,
}

#[derive(Copy, Clone)]
struct Quadrilateral {
    distance: f64,
    vertices: Matrix<4,2>,
    brightness: f64,
}

impl PartialEq for Quadrilateral {
    fn eq(&self, other: &Self) -> bool {
        (self.brightness - other.brightness).abs() < FLOAT_EPSILON &&
            (self.distance - other.distance).abs() < FLOAT_EPSILON &&
            self.vertices == other.vertices
    }
}

impl PartialEq for OrderedPiece {
    fn eq(&self, other: &Self) -> bool {
        (self.distance - other.distance).abs() < FLOAT_EPSILON
    }
}

impl Eq for OrderedPiece {}

impl PartialOrd for OrderedPiece {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for OrderedPiece {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance.partial_cmp(&other.distance).expect("Tried to order pieces when one of the distances was NaN")
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
        self.distance.partial_cmp(&other.distance).expect("Tried to order pieces when one of the distances was NaN")
    }
}

impl Quadrilateral {
    fn empty() -> Self {
        Quadrilateral {
            distance: 0.0,
            vertices: Matrix::ZERO(),
            brightness: 0.0
        }
    }
}

impl DrawablePiece {
    /// Returns an array of row matricies that correspond to the positions of the 
    /// pieces' vertices. 
    fn get_vertex_positions(&self) -> [Vec3; 8] {
        let r = self.radius;
        let c = self.center;
        let mut vertices = [Vec3::new(c.x, c.y, c.z); 8];
        for i in 0..8 {
            vertices[i] = match i {
                P_TOP_RIGHT_FRONT    => vertices[0] + Vec3::new( r, -r,  r),  
                P_TOP_RIGHT_BACK     => vertices[1] + Vec3::new( r,  r,  r),
                P_TOP_LEFT_BACK      => vertices[2] + Vec3::new(-r,  r,  r),
                P_TOP_LEFT_FRONT     => vertices[3] + Vec3::new(-r, -r,  r),
                P_BOTTOM_RIGHT_FRONT => vertices[4] + Vec3::new( r, -r, -r),  
                P_BOTTOM_RIGHT_BACK  => vertices[5] + Vec3::new( r,  r, -r),
                P_BOTTOM_LEFT_BACK   => vertices[6] + Vec3::new(-r,  r, -r),
                P_BOTTOM_LEFT_FRONT  => vertices[7] + Vec3::new(-r, -r, -r),
                _ => unreachable!("Vertex index not valid?"),
            };
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
        verts: [Vec3;8],
        projected_verts: Matrix<8,2>
    )-> [Quadrilateral;6] {
        // faces are the 
        let mut projected_faces:[Quadrilateral;6] = [Quadrilateral::empty();6];
        let mut faces: [[Vec3;4];6] = [[Vec3::ZERO;4];6];

        faces[0] = [verts[0],verts[1],verts[5],verts[4]];
        faces[1] = [verts[0],verts[3],verts[7],verts[4]];
        faces[2] = [verts[0],verts[1],verts[2],verts[3]];
        faces[3] = [verts[2],verts[3],verts[7],verts[6]];
        faces[4] = [verts[1],verts[2],verts[6],verts[5]];
        faces[5] = [verts[4],verts[5],verts[6],verts[7]]; 

        projected_faces[0].vertices = Matrix::<4,2>([projected_verts[0], projected_verts[1], projected_verts[5], projected_verts[4]]);
        projected_faces[1].vertices = Matrix::<4,2>([projected_verts[0], projected_verts[3], projected_verts[7], projected_verts[4]]);
        projected_faces[2].vertices = Matrix::<4,2>([projected_verts[0], projected_verts[1], projected_verts[2], projected_verts[3]]);
        projected_faces[3].vertices = Matrix::<4,2>([projected_verts[2], projected_verts[3], projected_verts[7], projected_verts[6]]);
        projected_faces[4].vertices = Matrix::<4,2>([projected_verts[1], projected_verts[2], projected_verts[6], projected_verts[5]]);
        projected_faces[5].vertices = Matrix::<4,2>([projected_verts[4], projected_verts[5], projected_verts[6], projected_verts[7]]);

        let center = Vec3::new(self.center.x,self.center.y, self.center.z);
        for i in 0..6 {
            projected_faces[i].distance = furthest_vertex_from_point(faces[i], camera_pos);

            let normal_vector = get_normal_vector(faces[i], center);
            let dot_product = normal_vector.dot_product(light_dir.normalize().unwrap());
            projected_faces[i].brightness = MIN_BRIGHTNESS_MULTIPLIER.max(dot_product*GENERAL_BRIGHTNESS_MULTIPLIER);
        }

        projected_faces
    }

    fn find_intersection(
        point: Vec3,
        camera_pos: Vec3,
        camera_dir: Vec3
    ) -> Option<Vec3> {
        let v = point - camera_pos;
        let n = camera_dir;
        let p1 = point;
        let p2 =  camera_pos + camera_dir*DISTANCE_CAMERA_PLANE;


        // System of equations to find intersection point.
        let mut eq: Matrix<3,4> = Matrix::<3,4>::ZERO();
        eq[0] = MatRow::<4>([v.y     , -v.x      ,  0.0      ,  p1.x*v.y - p1.y*v.x]);
        eq[1] = MatRow::<4>([0.0     , -v.z      ,  v.y      ,  p1.z*v.y - p1.y*v.z]);
        eq[2] = MatRow::<4>([n.x     ,  n.y      ,  n.z      ,  p2.x*n.x + p2.y*n.y + p2.z*n.z]);
        
        // Gaussian elimination (Don't look at this please. I just copy pasted from my old code '>_>)
        eq[1] = eq[0][0]*eq[1] - eq[1][0]*eq[0];
        eq[2] = eq[0][0]*eq[2] - eq[2][0]*eq[0];
        eq[2] = eq[1][1]*eq[2] - eq[2][1]*eq[1];

        let mut result: Vec3 = Vec3::ZERO;
        if eq[0][0] == 0.0 || eq[1][1] == 0.0 || eq[2][2] == 0.0 {
            return None;
        }
        result.z = eq[2][3]/eq[2][2];
        result.y = (eq[1][3] - eq[1][2]*result.z)/eq[1][1];
        result.x = (eq[0][3] - eq[0][2]*result.z - eq[0][1]*result.y)/eq[0][0];

        Some(result)
    }


    fn to_xy_plane(vertices: [Vec3; 8], camera_pos: Vec3, camera_dir: Vec3) -> Matrix<8,2> {
        let n = camera_dir;
        let basis1 = Vec3::new(
            n.y*0.0 - n.z*n.y,
            n.z*n.x - n.x*0.0,
            n.x*n.y - n.y*n.x
        );
        let n_i = basis1.normalize().unwrap();

        let basis2 = Vec3::new(
            n.y*basis1.z - n.z*basis1.y,
            n.z*basis1.x - n.x*basis1.z,
            n.x*basis1.y - n.y*basis1.x);
        let n_j = basis2.normalize().unwrap();
        
        let transformation = Matrix::<3,3>([
            MatRow::<3>([n_i.x, n_j.x, n.x]),
            MatRow::<3>([n_i.y, n_j.y, n.y]),
            MatRow::<3>([n_i.z, n_j.z, n.z]),
        ]);

        let inverse_transformation:Matrix<3,3> = transformation.inverse().unwrap();
        let mut result = Matrix::<8,2>::ZERO();

        let aux = camera_pos + camera_dir*DISTANCE_CAMERA_PLANE;
        let cam_projection = MatRow::<3>([aux.x, aux.y, aux.z]);

        for (i, v) in vertices.iter().enumerate() {
            let column_input = (Matrix::<1,3>([MatRow::<3>([v.x,v.y,v.z])- cam_projection])).transpose();
            let column = inverse_transformation*column_input;
            result[i] = MatRow::<2>([column[0][0],column[1][0]]);
        }
        result
    }

    fn project_points(vertices: [Vec3; 8], camera_pos: Vec3, camera_dir: Vec3) -> Matrix<8,2> {
        let mut intersections: [Vec3; 8] = [Vec3::ZERO;8];
        for i in 0..8 {
            let vec = vertices[i] - camera_pos;
            let intersection_option = Self::find_intersection(vertices[i], camera_pos, camera_dir);
            intersections[i] = match intersection_option {
                Some(x) => x,
                None    => Vec3::new(0.0,0.0,0.0),
            };
        }
        Self::to_xy_plane(intersections,camera_pos,camera_dir)
    }

    fn draw(&self, camera_pos: Vec3, camera_dir: Vec3, light_dir: Vec3) -> String {
        let vertices = self.get_vertex_positions();
        let projected_vertices = Self::project_points(vertices, camera_pos, camera_dir);

        let mut projected_faces = self.get_polygons_with_brightness(light_dir, camera_pos, vertices, projected_vertices);
        // Sorts the polygons from furthest to nearest.
        projected_faces.sort_by(|a,b| b.cmp(a));
        
        let mut buffer = String::new();

        
        for face in projected_faces {
            buffer.push_str("<polygon points=\"");
            for i in 0..4 {
                let x:usize = (face.vertices[i][0]*100.0 + 0.5*WIDTH  as f64) as usize;
                let y:usize = (face.vertices[i][1]*100.0 + 0.5*HEIGHT as f64) as usize;
                buffer.push_str(&format!("{x},{y} "));
            }
            let color: usize = (face.brightness*255.0) as usize;
            buffer.push_str(&format!("\" fill=\"#{color:02x}{color:02x}{color:02x}\" stroke=\"none\"/>\n"));
        }

        buffer
    }
}

fn get_normal_vector(face: [Vec3; 4], center: Vec3) -> Vec3 {
    dbg!(face);
    let normal = (face[1] - face[0]).cross_product( face[2] - face[0]).normalize().unwrap();
    let dot_product = normal.dot_product(center-face[0]);

    normal * if dot_product < 0.0 {-1.0} else {1.0}
}

#[derive(Clone, Debug, Default, Copy)]
struct DrawableCube {
    pieces: [DrawablePiece; 8],
}

const DRAWING_PIECE_RADIUS: f64 = 10.0;
impl Cube2 {
    fn to_points(self) -> DrawableCube {
        let r = DRAWING_PIECE_RADIUS;
        let mut drawable_pieces = [DrawablePiece::default(); 8 ];

        for (piece_idx, original_piece) in self.pieces.iter().enumerate() {
            let rotation: PieceRotation = original_piece.rotation;
            let center: Point = match piece_idx {
                P_TOP_RIGHT_FRONT    => Point::new( r, -r,  r),  
                P_TOP_RIGHT_BACK     => Point::new( r,  r,  r),
                P_TOP_LEFT_BACK      => Point::new(-r,  r,  r),
                P_TOP_LEFT_FRONT     => Point::new(-r, -r,  r),
                P_BOTTOM_RIGHT_FRONT => Point::new( r, -r, -r),  
                P_BOTTOM_RIGHT_BACK  => Point::new( r,  r, -r),
                P_BOTTOM_LEFT_BACK   => Point::new(-r,  r, -r),
                P_BOTTOM_LEFT_FRONT  => Point::new(-r, -r, -r),
                _ => unreachable!("Piece index no vàlid?"),
            };

            drawable_pieces[piece_idx] = DrawablePiece {
                center,
                radius: DRAWING_PIECE_RADIUS,
                rotation
            };
        }

        DrawableCube { pieces: drawable_pieces }
    }
}

/// Draw the solving sequence, with `n_in_between_frames`*`moves.len()` frames
pub fn draw_sequence(file_prefix: &str, starting_cube: &Cube2, moves: &[Move], n_in_between_frames: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut cube: Cube2 = *starting_cube;

    for (i, mov) in moves.iter().enumerate() {
        let i = i * n_in_between_frames;
        for inbetween_index in 0..n_in_between_frames {
            let lerp_t = inbetween_index as f64 / n_in_between_frames as f64;
            let filename = format!("{file_prefix}_{:>04}", i + inbetween_index);

            let svg: String = get_svg(cube, *mov, lerp_t);
            
            let mut file: fs::File = fs::File::create(filename)?;
            file.write_all(svg.as_bytes())?;
        }
        cube.make_move(*mov);
    }

    todo!()

}

/// Given a cube, the move being done and how far along the move is, generate the corresponding svg as a String. This is a self-contained frame representing the cube in the given state.
fn get_svg(cube: Cube2, mov: Move, lerp_t: f64) -> String {
    let pieces = cube.to_points().pieces; // Un array de 8 DrawablePieces, que contenen els seus punts
    // Recorda que el radi és DRAWING_PIECE_RADIUS
    format!("{cube} with {mov:?} at with lerp value {lerp_t}");
    let cam_pos = Vec3::new(20.0, 20.0, 20.0)*10.0;
    let cam_dir = Vec3::ZERO - cam_pos;

    let light_pos = Vec3::new(10.0,20.0,-30.0);
    let light_dir = Vec3::ZERO - light_pos;

    let mut ordered_pieces:[OrderedPiece;8] = [OrderedPiece::default();8];
    
    for (i, piece) in pieces.iter().enumerate() {
        let distance = (Vec3::new(piece.center.x, piece.center.y, piece.center.z) - cam_pos).abs();
        ordered_pieces[i] = OrderedPiece{distance, piece: *piece};
    }

    ordered_pieces.sort_by(|a, b| b.cmp(a));


    let mut buffer: String = String::new();

    buffer.push_str(&format!("<svg viewBox=\"0 0 {WIDTH} {HEIGHT} \" xmlns=\"http://www.w3.org/2000/svg\" id=\"vonkoch-holder\">\n"));


    for piece in ordered_pieces {
        buffer.push_str(&piece.piece.draw(cam_pos, cam_dir, light_dir));
    }

    buffer.push_str("</svg>\n");

    buffer
}


fn furthest_vertex_from_point(vertices: [Vec3;4], point: Vec3) -> f64 {
    let mut max_dist: f64 = 0.0;
    for vertex in vertices {
        let dist = (vertex - point).abs();
        if dist > max_dist { max_dist = dist }
    }
    max_dist
}

#[test]
fn test_drawing_piece() {
    let piece = DrawablePiece{rotation: PieceRotation::WB, center: Point{x:5.0,y:5.0,z:5.0}, radius:5.0};

    let cam_pos = (Vec3::new(35.5, 25.0, 10.5))*10.0;
    let cam_dir = Vec3::ZERO - cam_pos;

    let light_pos = Vec3::new(12.0,20.2,30.7);
    let light_dir = Vec3::ZERO - light_pos;

    let buffer = piece.draw(cam_pos, cam_dir, light_dir);
    println!("{}", buffer);
}

#[test]
fn test_drawing_cube() {
    let cube = Cube2::default();
    let m = Move{side:MoveSide::R, prime: false};
    
    let text = get_svg(cube,m,0.0);
    println!("{}", text);
}
