use m_per_n::*;
use crate::*;

use std::fs::{self, File};
use std::io::Write;
use m_per_n::Vec3;

#[derive(Clone, Debug, Default, Copy)]
struct DrawablePiece {
    rotation: PieceRotation,
    center: Point,
    radius: f64,
}

impl DrawablePiece {

    /// Returns an array of row marices which correspond to the positions of the 
    /// pieces' vertices. 
    fn get_vertex_positions(&self) -> [MatRow<3>; 8] {
        let r = self.radius;
        let c = self.center;
        let mut vertices = [MatRow::<3>([c.x,c.y,c.z]);8];

        for i in 0..7 {
            let point = match i {
                P_TOP_RIGHT_FRONT    => vertices[0] + MatRow::<3>([ r, -r,  r]),  
                P_TOP_RIGHT_BACK     => vertices[1] + MatRow::<3>([ r,  r,  r]),
                P_TOP_LEFT_BACK      => vertices[2] + MatRow::<3>([-r,  r,  r]),
                P_TOP_LEFT_FRONT     => vertices[3] + MatRow::<3>([-r, -r,  r]),
                P_BOTTOM_RIGHT_FRONT => vertices[4] + MatRow::<3>([ r, -r, -r]),  
                P_BOTTOM_RIGHT_BACK  => vertices[5] + MatRow::<3>([ r,  r, -r]),
                P_BOTTOM_LEFT_BACK   => vertices[6] + MatRow::<3>([-r,  r, -r]),
                P_BOTTOM_LEFT_FRONT  => vertices[7] + MatRow::<3>([-r, -r, -r]),
                _ => unreachable!("Piece index no vàlid?"),
            };
            vertices[i] = point;
        }
        vertices
    }

    /// Returns an array of 3x3 matrices which represent triplets of coordinates which 
    /// make up the faces of the cube. (in the order described during hack nights!)
    fn get_faces_with_brightness(&self, light_dir: Vec3, verts: &mut [MatRow<3>;8]) -> [(Matrix<4,3>, f64);6] {
        *verts = self.get_vertex_positions();
        let mut faces:[(Matrix<4,3>, f64);6] = [(Matrix::ZERO(), 0.0);6];
        faces[0].0 = Matrix::<4,3>([verts[0], verts[1], verts[4],verts[5]]);
        faces[1].0 = Matrix::<4,3>([verts[0], verts[3], verts[4],verts[7]]);
        faces[2].0 = Matrix::<4,3>([verts[0], verts[1], verts[2],verts[3]]);
        faces[3].0 = Matrix::<4,3>([verts[2], verts[3], verts[6],verts[7]]);
        faces[4].0 = Matrix::<4,3>([verts[1], verts[2], verts[5],verts[6]]);
        faces[5].0 = Matrix::<4,3>([verts[4], verts[5], verts[6],verts[7]]);

        let center = Vec3::new(self.center.x,self.center.y, self.center.z);
        for (face, brightness) in &mut faces {
            let normal_vector = get_normal_vector(*face, center);
            *brightness = 0.8*max(0.1, Vec3::dot_product(normal_vector, light_dir));
        }
        faces
    }

    fn draw(&self, light_dir: Vec3) -> String {
        let vertices: &mut [MatRow<3>;8] = &mut [MatRow::<3>([0.0,0.0,0.0]);8];
        let cube_faces = self.get_faces_with_brightness(light_dir, vertices);
        
        todo!()
    }
}

fn get_normal_vector(face: Matrix<4,3>, center: Vec3) -> Vec3 {
    let vertex0: Vec3 = Vec3::new(face[0][0], face[0][1],face[0][2]);
    let vertex1: Vec3 = Vec3::new(face[1][0], face[1][1],face[1][2]);
    let vertex2: Vec3 = Vec3::new(face[2][0], face[2][1],face[2][2]);

    let mut normal = Vec3::cross_product(vertex1 - vertex0, vertex2 - vertex0);
    normal = normal.normalize().unwrap();
    let dot_product = Vec3::dot_product(normal,center-vertex0);

    normal * if dot_product < 0.0 {-1.0} else {1.0}
}

#[derive(Clone, Debug, Default, Copy)]
struct DrawableCube {
    pieces: [DrawablePiece; 8],
}

const DRAWING_PIECE_RADIUS: f64 = 10.0;
impl Cube {
    fn to_points(&self) -> DrawableCube {
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

pub fn draw_sequence(file_prefix: &str, starting_cube: &Cube, moves: Vec<Move>, n_in_between_frames: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut cube: Cube = starting_cube.clone();

    for (i, mov) in moves.iter().enumerate() {
        let i = i * n_in_between_frames;
        for inbetween_index in 0..n_in_between_frames {
            let lerp_t = inbetween_index as f64 / n_in_between_frames as f64;
            let filename = format!("{file_prefix}_{:>04}", i + inbetween_index);

            let svg: String = get_svg(&cube, &mov, lerp_t);
            
            let mut file: fs::File = fs::File::create(filename)?;
            file.write(svg.as_bytes())?;
        }
        cube.make_move(mov);
    }

    todo!()

}

fn get_svg(cube: &Cube, mov: &Move, lerp_t: f64) -> String {
    let points = cube.to_points().pieces; // Un array de 8 DrawablePieces, que contenen els seus punts
    
    // Recorda que el radi és DRAWING_PIECE_RADIUS
    format!("{cube} with {mov:?} at with lerp value {lerp_t}");
    let cam_pos = Vec3::new(10.0, 10.0, 10.0);
    let cam_dir = Vec3::ZERO - cam_pos;

    let light_pos = Vec3::new(0.0,0.0,20.0);
    let light_dir = Vec3::ZERO - light_pos;




    todo!();
}
