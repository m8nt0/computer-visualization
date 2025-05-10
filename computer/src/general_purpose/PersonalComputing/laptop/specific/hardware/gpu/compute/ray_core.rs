use super::super::error::{GPUError, GPUResult};

pub struct RayCore {
    id: u32,
    state: CoreState,
    bvh: Option<BVHTree>,
    rays: Vec<Ray>,
    stats: RayStats,
}

struct Ray {
    origin: Vector3,
    direction: Vector3,
    t_min: f32,
    t_max: f32,
}

struct BVHTree {
    nodes: Vec<BVHNode>,
    primitives: Vec<Primitive>,
}

struct BVHNode {
    bounds: AABB,
    left: Option<usize>,
    right: Option<usize>,
    primitive_index: Option<usize>,
}

impl RayCore {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            state: CoreState::Idle,
            bvh: None,
            rays: Vec::new(),
            stats: RayStats::default(),
        }
    }

    pub fn trace_rays(&mut self, rays: Vec<Ray>) -> GPUResult<Vec<Intersection>> {
        self.state = CoreState::Active;
        self.rays = rays;
        self.stats.rays_started += self.rays.len();

        let mut intersections = Vec::new();
        for ray in &self.rays {
            if let Some(hit) = self.trace_ray(ray) {
                intersections.push(hit);
                self.stats.rays_hit += 1;
            }
        }

        Ok(intersections)
    }

    fn trace_ray(&self, ray: &Ray) -> Option<Intersection> {
        if let Some(bvh) = &self.bvh {
            self.traverse_bvh(ray, &bvh.nodes[0])
        } else {
            None
        }
    }

    fn traverse_bvh(&self, ray: &Ray, node: &BVHNode) -> Option<Intersection> {
        if !node.bounds.intersect(ray) {
            return None;
        }

        // Leaf node - test primitive intersection
        if let Some(prim_idx) = node.primitive_index {
            return self.intersect_primitive(ray, &self.bvh.as_ref().unwrap().primitives[prim_idx]);
        }

        // Internal node - recurse
        let left_hit = node.left.map(|idx| self.traverse_bvh(ray, &self.bvh.as_ref().unwrap().nodes[idx])).flatten();
        let right_hit = node.right.map(|idx| self.traverse_bvh(ray, &self.bvh.as_ref().unwrap().nodes[idx])).flatten();

        // Return closest intersection
        match (left_hit, right_hit) {
            (Some(l), Some(r)) => Some(if l.t < r.t { l } else { r }),
            (Some(h), None) | (None, Some(h)) => Some(h),
            (None, None) => None,
        }
    }
}