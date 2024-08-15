use crate::*;
use shared::Color;
use geo::{Polygon, LineString, Coord, BooleanOps, CoordsIter, Centroid};
use m_per_n::Vec3;
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

        let mut normal_vec = (vertices[1]._3d - vertices[0]._3d)
        .cross_product(vertices[2]._3d - vertices[0]._3d)
        .normalize()
        .unwrap();
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

    pub fn get_overlap_centroid_2d(&self, other: Stiker, camera: Camera) -> Option<geo::Point> {
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

    pub fn is_in_front(&self, st: Stiker, camera: Camera) -> Option<Ordering>{
        todo!();
    }

    pub fn get_drawing_data(&self) {
        todo!();
    }
}