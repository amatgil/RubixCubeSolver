use crate::*;
use m_per_n::Vec3;
use shared::{Move};
use tubaitu::{Cube2};
use std::cmp::Ordering;
use std::collections::VecDeque;


#[derive(Copy, Clone)]
struct DepthNode {
    index: usize,
    depth: usize,
}

pub struct DrawableCube2 {
    pieces: [DrawablePiece; 8],
    depth_map: [DepthNode; 8],
}

impl DrawableCube2 {
    pub fn new(cube: Cube2, mov: Move, lerp_t: f64) -> Self{

        let mut pieces: [DrawablePiece;8] = [DrawablePiece::new(Vec3::ZERO, 0.0 , PieceRotation::BO);8];
        let r = PIECE_RADIUS + EXTRA_PIECE_DISTANCE;

        for (i, piece_) in cube.pieces.iter().enumerate() {
            let center: Vec3 = match i {
                0 => Vec3::new( r, -r,  r),
                1 => Vec3::new( r,  r,  r),
                2 => Vec3::new(-r,  r,  r),
                3 => Vec3::new(-r, -r,  r),
                4 => Vec3::new( r, -r, -r),
                5 => Vec3::new( r,  r, -r),
                6 => Vec3::new(-r,  r, -r),
                7 => Vec3::new(-r, -r, -r),
                _ => panic!()
            };
            pieces[i] = DrawablePiece::new(center, PIECE_RADIUS , piece_.rotation);
        };
        let pieces_to_rotate = get_corner_cycle(mov);
        for i in pieces_to_rotate {
            pieces[i].apply_rotation(mov, lerp_t);
        }

        let mut depths = [DepthNode{depth: 0, index: 0}; 8];
        for i in 0..8 {
            depths[i].index = i;
        }

        DrawableCube2{
            pieces: pieces,
            depth_map: depths,
        }
    }

    pub fn generate_depth_map(&mut self, camera: &Camera) {
        // the (usize, usize) represents a directed edge: (from, to).
        let mut edges: Vec<(usize, usize)> = Default::default();
        let mut behind: [Vec<usize>; 8] = Default::default();

        // Generate the Polytree:
        for i in 0..(self.pieces.len()-1){
            for j in (i+1)..self.pieces.len() {
                let comp_option = self.pieces[i].cmp_dist_to_cam(self.pieces[j], &camera);

                if let Some(comp) = comp_option {
                    if(comp == Ordering::Less) {
                        behind[i].push(j);
                        edges.push((i, j));
                    } else {
                        behind[j].push(i);
                        edges.push((j, i));
                    }
                } else {
                }
            }
        }

        // Find the roots:
        let mut roots: Vec<usize> = Default::default(); 
        let mut in_degree: [usize; 8] = [0; 8];

        for &(_, to) in &edges {
            in_degree[to] += 1;
        }

        for i in 0..8 {
            if in_degree[i] == 0 {
                roots.push(i);
            }
        }
        let n_of_roots = roots.len();
        // Breadth first traversal: 
        let mut queue:  VecDeque<usize> = VecDeque::<usize>::from(roots);
        let mut depths: VecDeque<usize> = VecDeque::<usize>::from(vec![0; n_of_roots]);

        while(queue.len() != 0) {
            let node = queue.pop_front().unwrap();
            let depth = depths.pop_front().unwrap();
            for &face in &behind[node] {
                queue.push_back(face);
                depths.push_back(depth + 1);
                self.depth_map[face].depth = depth + 1;
            }
        }
    }

    pub fn project_pieces(&mut self, camera: &Camera, light_dir: Vec3) {
        for i in 0..self.pieces.len() {
            self.pieces[i].project_vertices(camera, light_dir);
        }
    }
    
    pub fn set_black_internals(&mut self) {
        for i in 0..self.pieces.len() {
            self.pieces[i].set_black_internals();
        }
    }

    pub fn get_drawing_data(&mut self, camera: &Camera, light_dir: Vec3) -> Vec<Quadrilateral> {
        self.project_pieces(&camera, light_dir);
        self.set_black_internals();
        self.generate_depth_map(&camera);

        let mut depth_map_copy = self.depth_map;
        depth_map_copy.sort_by(|x, y| (x.depth).cmp(&y.depth).reverse());

        let mut result: Vec<Quadrilateral> = Default::default();

        for node in depth_map_copy {
            result.append(&mut self.pieces[node.index].get_drawing_data(&camera));
        }

        result
    }
}