use crate::{
    hittable::{hit_record, Hittable},
    ray::Ray,
    vec3::Point3,
};

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    // Constructor for an empty HittableList
    pub fn new_empty() -> Self {
        HittableList {
            objects: Vec::new(),
        }
    }

    // Constructor that initializes the list with a single object
    pub fn new(object: Box<dyn Hittable>) -> Self {
        let mut list = HittableList::new_empty();
        list.add(object);
        list
    }

    // Method to add an object to the list
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut hit_record) -> bool {
        let mut temp_rec: hit_record = hit_record::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        hit_anything
    }
}
