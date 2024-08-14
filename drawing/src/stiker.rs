use crate::*;
use shared::Color;
use geo::{Coord, Polygon};
use m_per_n::Vec3;

#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub _3d: Vec3,
    pub _2d: Coord,
}

#[derive(Copy, Clone, Default)]
pub struct Stiker {
    pub vertices: [Vertex; 4],
    normal_vec: Vec3,
    color: Color,
    brightness: f64,
}

impl Stiker {
    pub fn new(vertices: [Vertex; 4], color: Color) -> Self {
        let mut center= Vec3::ZERO;
        for v in vertices {
            center = center + v._3d;
        }
        center = center*(1.0/vertices.len() as f64);

        let mut normal_vec = (vertices[1]._3d - vertices[0]._3d)
        .cross_product(vertices[2]._3d - vertices[0]._3d)
        .normalize()
        .unwrap();
        let dot_product = normal_vec.dot_product(center - vertices[0]._3d);

        normal_vec = normal_vec * if dot_product < 0.0 { -1.0 } else { 1.0 };

        Stiker{
            vertices: vertices,
            normal_vec: normal_vec,
            color: color,
            brightness: 0.0,
        }
    }

    pub fn update_brightness(&self, light_dir: Vec3) {
        todo!();
    }

    pub fn get_polygon(&self) -> Polygon {
        todo!();
    }

    pub fn get_overlap_centroid_2d(&self, quad: Stiker, camera: Camera) -> Option<Coord> {
        todo!();
    }

    pub fn get_drawing_data(&self) {
        todo!();
    }
}