use crate::*;
use m_per_n::*;
use shared::*;

use m_per_n::Vec3;
use std::cmp::Ordering;
use std::f64::consts::PI; // The lesser cercle constant


impl Drawable<8> for Cube2 {
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
