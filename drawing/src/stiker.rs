use crate::*;
use shared::Color;
use geo::{Polygon, LineString, Coord, BooleanOps, CoordsIter, Centroid};
use m_per_n::{Vec3, Matrix, MatRow};
use std::cmp::{Ordering};

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub _3d: Vec3,
    pub _2d: Coord::<f64>,
}

#[derive(Copy, Clone, Default)]
pub struct Stiker {
    pub vertices: [Vertex; 4],
    normal_vec: Vec3,
    color: Color,
    brightness: f64,
}

impl Stiker {
    pub fn new(vertices: [Vertex; 4], color: Color, piece_center: Vec3) -> Self {

        let mut normal_vec = if let Some(thingy) = (vertices[1]._3d - vertices[0]._3d)
        .cross_product(vertices[2]._3d - vertices[0]._3d)
        .normalize() {
            (vertices[1]._3d - vertices[0]._3d).cross_product(vertices[2]._3d - vertices[0]._3d).normalize().unwrap()
        }
        else {Vec3::ZERO};

        let dot_product = normal_vec.dot_product(piece_center - vertices[0]._3d);

        normal_vec = normal_vec * if dot_product < 0.0 { -1.0 } else { 1.0 };

        Stiker{
            vertices: vertices,
            normal_vec: normal_vec,
            color: color,
            brightness: 0.0,
        }
    }

    pub fn update_brightness(&mut self, light_dir: Vec3) {
        let dot_product = self.normal_vec.dot_product(light_dir.normalize().unwrap());
        self.brightness = MIN_BRIGHTNESS_MULTIPLIER.max(dot_product * GENERAL_BRIGHTNESS_MULTIPLIER);
    }

    pub fn get_polygon(&self) -> Polygon::<f64> {
        let mut verts_2d = [Coord::<f64>::zero(); 4];
        for i in 0..self.vertices.len() {
            verts_2d[i] = self.vertices[i]._2d;
        }
        Polygon::<f64>::new(LineString::from(Vec::from(verts_2d)), vec![])
    }

    pub fn get_overlap_centroid_2d(&self, other: &Stiker) -> Option<geo::Point> {
        let poly1 = self.get_polygon();
        let poly2 = other.get_polygon();
        
        let intersection = poly1.intersection(&poly2);

        // Return the center of the first polygon from the resulting geometry collection
        if let Some(result) = intersection.into_iter().next() {
            return result.centroid();
        }
        else {
            return None;
        }
    }

    pub fn cmp_dist_to_cam(&self, other: Stiker, camera: &Camera) -> Option<Ordering>{
        if let Some(overlap_center) = self.get_overlap_centroid_2d(&other) {

            let mat = camera.get_from_xy_to_xyz_matrix();
            let input = MatRow::<3>([overlap_center.x(), overlap_center.y(), 0.0]);
            let p = mat*Matrix::<1,3>([input; 1]).transpose();

            let r = Ray {
                point: camera.position,
                direction: Vec3::new(p[0][0],- p[1][0], p[2][0]) - camera.position,
            };
            
            let inter1 = self.intersection_with_ray(&r);
            let inter2 = other.intersection_with_ray(&r);
            let t1 = (inter1 - camera.position).abs();
            let t2 = (inter2 - camera.position).abs();
            
            if ((t1 - t2).abs() < FLOAT_EPSILON) {return Some(Ordering::Equal);}
            else if (t1 < t2) {return Some(Ordering::Less)}
            else {return Some(Ordering::Greater)}
        }
        else {
            return None;
        }
        
    }

    pub fn intersection_with_ray(&self, ray: &Ray) -> Vec3{
        let N = self.normal_vec;
        let V = ray.direction;
        let P_0 = ray.point;
        let d = -N.dot_product(self.vertices[0]._3d);
        let t = -(P_0.dot_product(N) + d)/V.dot_product(N);
        return P_0 + V*t;
    }

    pub fn get_drawing_data(&self) -> Quadrilateral {
        let mut quad = Quadrilateral::default();
        
        for (i, vert) in self.vertices.iter().enumerate() {
            quad.vertices[i] = vert._2d;
        }
        quad.color = self.color.to_rgb(self.brightness);

        quad
    }
}