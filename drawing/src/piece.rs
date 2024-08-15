use crate::*;
use shared::PieceRotation;
use geo::{Polygon, LineString, Coord, BooleanOps, CoordsIter, Centroid};
use m_per_n::{Vec3, MatRow, Matrix};
use std::collections::VecDeque;
use std::cmp::Ordering;
use std::f64::consts::PI;

#[derive(Copy, Clone)]
struct DepthNode {
    index: usize,
    priority: usize,
}

#[derive(Copy, Clone)]
pub struct DrawablePiece {
    vertices: [Vertex; 8],
    faces: [Stiker; 6],
    depth_map: [DepthNode; 6],
}

impl DrawablePiece {
    pub fn new(center: Vec3, radius: f64, rotation:PieceRotation) -> Self{
        let mut vertices: [Vertex; 8] = [Vertex::default(); 8];
        
        let r = radius;

        for i in 0..vertices.len() {
            vertices[i]._3d = match i {
                0 => Vec3::new( r, -r,  r),
                1 => Vec3::new( r,  r,  r),
                2 => Vec3::new(-r,  r,  r),
                3 => Vec3::new(-r, -r,  r),
                4 => Vec3::new( r, -r, -r),
                5 => Vec3::new( r,  r, -r),
                6 => Vec3::new(-r,  r, -r),
                7 => Vec3::new(-r, -r, -r),
                _ => todo!(),
            }
        }

        let mut faces: [[Vertex; 4]; 6] = [[Vertex::default(); 4]; 6];

        for i in 0..faces.len() {
            let positions = get_vertices_in_face(i);
            for j in 0..positions.len() {
                faces[i][j] = vertices[positions[j]];
            }
        }

        let mut stikers: [Stiker; 6] = [Stiker::default(); 6];

        for i in 0..stikers.len() {
            stikers[i] = Stiker::new(faces[i], rotation.to_color_sequence()[i], center);
        }

        let mut depths = [DepthNode{priority: 0,index: 0}; 6];
        for i in 0..6 {
            depths[i].index = i;
        }

        DrawablePiece{
            vertices:vertices,
            faces: stikers,
            depth_map: depths,
        }
    }


    pub fn apply_rotation(&mut self, mov: Move, mut lerp_t: f64) {
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

        for i in 0..self.vertices.len() {
            let col_vec: Matrix<3, 1> = self.vertices[i]._3d.into(); // Write as column vector
            self.vertices[i]._3d = (matrix * col_vec).into();
        }

        self.update_stikers();

    }


    pub fn project_vertices(&mut self, camera: &Camera, light_dir: Vec3) {

        let transformation = camera.get_from_xyz_to_xy_matrix();
        for i in 0..self.vertices.len() {

            let p = camera.intersection_with_plane(self.vertices[i]._3d);
            let transformed = transformation * Matrix::<1,3>([MatRow::<3>([p.x, p.y, p.z])]).transpose();
            self.vertices[i]._2d = geo::coord!{x: transformed[0][0], y: transformed[1][0]};
        }

        for i in 0..self.faces.len() {
            self.faces[i].update_brightness(light_dir);
        }

        self.update_stikers();
    }


    pub fn update_stikers(&mut self) {
        for i in 0..self.faces.len() {
            let positions = get_vertices_in_face(i);
            for j in 0..positions.len() {
                self.faces[i].vertices[j] = self.vertices[j];
            }
        }
    }


    pub fn generate_depth_map(&mut self, camera: &Camera) {
        // the (usize, usize) represents a directed edge: (from, to).
        let mut edges: Vec<(usize, usize)> = Default::default();
        let mut behind: [Vec<usize>; 6] = Default::default();

        // Generate the Polytree:
        for i in 0..(self.faces.len()-1){
            for j in (i+1)..self.faces.len() {
                let comp_option = self.faces[i].cmp_dist_to_cam(self.faces[j], &camera);

                if let Some(comp) = comp_option {
                    if(comp == Ordering::Less) {
                        behind[i].push(j);
                        edges.push((i, j));
                    } else {
                        behind[j].push(i);
                        edges.push((j, i));
                    }
                }
            }
        }

        // Find the roots:
        let mut roots: Vec<usize> = Default::default(); 
        let mut in_degree: [usize; 6] = [0; 6];

        for &(from, to) in &edges {
            in_degree[to] += 1;
        }

        for i in 0..6 {
            if in_degree[i] == 0 {
                roots.push(i);
            }
        }

        // Breadth first traversal: 
        let mut queue: VecDeque<usize> = VecDeque::<usize>::from(roots);
        let mut depth = 0;

        while(queue.len() != 0) {
            let node = queue.pop_front().unwrap();

            for &face in &behind[node] {
                queue.push_back(face);
                self.depth_map[face].priority = depth;
            }
            depth += 1;
        }
    }


    pub fn get_outline_polygon(&self) -> Polygon {
        todo!();
    }

    pub fn get_overlap_centroid_2d(&self, piece: DrawablePiece, camera: Camera) -> Option<Coord> {
        todo!();
    }

    pub fn get_intersections_with_ray(&self, ray: Ray) -> Vec<Vec3> {
        todo!();
    }

    pub fn is_in_front(&self, piece: DrawablePiece, camera: Camera) -> Option<Ordering> {
        todo!();
    }

    pub fn get_drawing_data(&mut self, camera: &Camera, light_dir: Vec3) -> Vec<Quadrilateral> {
        self.project_vertices(&camera, light_dir);
        self.generate_depth_map(&camera);

        let mut depth_map_copy = self.depth_map;
        depth_map_copy.sort_by(|x, y| (x.priority).cmp(&y.priority));

        let mut result: Vec<Quadrilateral> = Default::default();

        for node in depth_map_copy {
            result.push(self.faces[node.index].get_drawing_data());
        }

        result
    }
}

fn get_vertices_in_face(face: usize) -> [usize; 4] {
    match face {
        SIDE_RIGHT  => FACE_RIGHT_SEQ_CYCLE,
        SIDE_LEFT  => FACE_LEFT_SEQ_CYCLE,
        SIDE_TOP    => FACE_UP_SEQ_CYCLE,
        SIDE_DOWN   => FACE_DOWN_SEQ_CYCLE,
        SIDE_FRONT   => FACE_FRONT_SEQ_CYCLE,
        SIDE_BACK   => FACE_BACK_SEQ_CYCLE,
        _ => panic!(),
    }
}