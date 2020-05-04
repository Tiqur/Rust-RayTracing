use crate::Classes::Vec3::Vec3;
use crate::PathTracing::Ray::Ray;

pub struct HitRecord {
    pub closest_point: Vec3,
    pub normal: Vec3,
    pub distance: f32,
    pub hit: bool
}

impl Default for HitRecord {
    fn default() -> HitRecord {
        HitRecord {
            closest_point: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            normal: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0
            },
            distance: -1.0,
            hit: false
        }
    }
}