use crate::*;
use m_per_n::*;
use shared::*;

use m_per_n::Vec3;
use std::cmp::Ordering;
use std::f64::consts::PI; // The lesser cercle constant


impl Drawable<8> for Cube2 {
    type DrawablePiece = DrawablePiece;
    fn to_points(self) -> [DrawablePiece; 8] {
        let r = DRAWING_PIECE_RADIUS + EXTRA_PIECE_DISTANCE;
        let mut drawable_pieces = [DrawablePiece::default(); 8];

        for (piece_idx, original_piece) in self.pieces.iter().enumerate() {
            let rotation: PieceRotation = original_piece.rotation;
            let center: Point = match piece_idx.try_into().unwrap() {
                PiecePosition::TopRightFront    => Point::new(r, -r, r),
                PiecePosition::TopRightBack     => Point::new(r, r, r),
                PiecePosition::TopLeftBack      => Point::new(-r, r, r),
                PiecePosition::TopLeftFront     => Point::new(-r, -r, r),
                PiecePosition::BottomRightFront => Point::new(r, -r, -r),
                PiecePosition::BottomRightBack  => Point::new(r, r, -r),
                PiecePosition::BottomLeftBack   => Point::new(-r, r, -r),
                PiecePosition::BottomLeftFront  => Point::new(-r, -r, -r),
            };

            drawable_pieces[piece_idx] = DrawablePiece {
                center,
                radius: DRAWING_PIECE_RADIUS,
                rotation,
                should_rotate: false,
            };
        }

        drawable_pieces
    }

    /// Given a cube, the move being done and how far along the move is, generate the corresponding polys that would draw it
    fn get_polys(&self, part_mov: Option<PartialMove>, width: usize, height: usize, scale: f64) -> Vec<Polygon> {
        let mut pieces = self.to_points(); // Un array de 8 DrawablePieces, que contenen els seus punts
        // Recorda que el radi és DRAWING_PIECE_RADIUS

        let (mov, lerp_t) = 
            if let Some(yougottamoveitmoveit) = part_mov {
                (yougottamoveitmoveit.mov, yougottamoveitmoveit.lerp_t)
            } else { (Move::R, 0.0) };

        let light_pos = Vec3::new(10.1, -20.1, 30.1);
        let light_dir = Vec3::ZERO - light_pos;

        let pos = Vec3::new(10.1, -30.1, 10.1) * 10.0;

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
            pieces[*i].should_rotate = true;
        }

        let mut projected_cube: [Quadrilateral; 48] = [Quadrilateral::empty(); 48];

        for (i, piece) in pieces.iter().enumerate() {
            let aux = piece.draw(camera, light_dir, mov, lerp_t);
            for j in 0..6 {
                projected_cube[6 * i + j] = aux[j];
            }
        }

        projected_cube.sort_by(|a, b| a.cmp(b).reverse());

        let mut buffer = vec![];

        for face in projected_cube {
            let mut polygon_points = vec![];
            for i in 0..4 {
                let x: usize = (face.vertices[i][0] * scale + 0.5 * width as f64) as usize;
                let y: usize = (face.vertices[i][1] * scale + 0.5 * height as f64) as usize;
                polygon_points.push((x, y));
            }

            buffer.push(Polygon {
                points: polygon_points,
                color: face.color.to_rgb(face.brightness),
            });
        }

        buffer
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
                PiecePosition::TopRightFront    => vertices[0] + Vec3::new(r, -r, r),
                PiecePosition::TopRightBack     => vertices[1] + Vec3::new(r, r, r),
                PiecePosition::TopLeftBack      => vertices[2] + Vec3::new(-r, r, r),
                PiecePosition::TopLeftFront     => vertices[3] + Vec3::new(-r, -r, r),
                PiecePosition::BottomRightFront => vertices[4] + Vec3::new(r, -r, -r),
                PiecePosition::BottomRightBack  => vertices[5] + Vec3::new(r, r, -r),
                PiecePosition::BottomLeftBack   => vertices[6] + Vec3::new(-r, r, -r),
                PiecePosition::BottomLeftFront  => vertices[7] + Vec3::new(-r, -r, -r),
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
            verts[*P::TopRightFront],    verts[*P::TopRightBack],     verts[*P::BottomRightBack], verts[*P::BottomRightFront]
        ];
        faces[*Side::Front] = [
            verts[*P::TopRightFront],    verts[*P::BottomRightFront], verts[*P::BottomLeftFront], verts[*P::TopLeftFront]
        ];
        faces[*Side::Top] =  [
            verts[*P::TopRightFront],    verts[*P::TopLeftFront],     verts[*P::TopLeftBack],     verts[*P::TopRightBack]
        ];
        faces[*Side::Left] = [
            verts[*P::TopLeftFront],     verts[*P::BottomLeftFront],  verts[*P::BottomLeftBack],  verts[*P::TopLeftBack]
        ];
        faces[*Side::Back] = [
            verts[*P::TopRightBack],     verts[*P::TopLeftBack],      verts[*P::BottomLeftBack],  verts[*P::BottomRightBack], 
            ];
        faces[*Side::Down] = [
            verts[*P::BottomRightFront], verts[*P::BottomRightBack],  verts[*P::BottomLeftBack],  verts[*P::BottomLeftFront]
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
