use std::io::empty;

use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

#[derive(Clone, Copy)]
pub struct hit_record {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl hit_record {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        if self.front_face {
            self.normal = *outward_normal
        } else {
            self.normal = -*outward_normal
        }
    }

    pub fn default() -> hit_record {
        hit_record {
            p: Point3::new_empty(),
            normal: Vec3::new_empty(),
            t: 0.0,
            front_face: true,
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &crate::ray::Ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool;
}
