use super::super::error::{GPUError, GPUResult};
use super::super::compute::{Vec3, Vec4, Mat4};
use super::framebuffer::Framebuffer;

pub struct Rasterizer {
    viewport: Viewport,
    depth_buffer: Vec<f32>,
    clip_planes: Vec<Plane>,
    transform: Transform,
    state: RasterState,
    stats: RasterStats,
}

struct Viewport {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    min_depth: f32,
    max_depth: f32,
}

struct Transform {
    model: Mat4,
    view: Mat4,
    projection: Mat4,
    viewport: Mat4,
}

struct Plane {
    normal: Vec3,
    distance: f32,
}

#[derive(Clone)]
struct RasterState {
    depth_test: bool,
    depth_write: bool,
    cull_face: CullFace,
    polygon_mode: PolygonMode,
    blend_mode: BlendMode,
}

#[derive(Clone, Copy)]
enum CullFace {
    None,
    Front,
    Back,
    Both,
}

#[derive(Clone, Copy)]
enum PolygonMode {
    Point,
    Line,
    Fill,
}

#[derive(Clone, Copy)]
enum BlendMode {
    None,
    Alpha,
    Additive,
    Multiply,
}

struct RasterStats {
    triangles_submitted: u64,
    triangles_culled: u64,
    fragments_generated: u64,
    fragments_passed: u64,
}

impl Rasterizer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            viewport: Viewport::new(0, 0, width, height),
            depth_buffer: vec![1.0; (width * height) as usize],
            clip_planes: Vec::new(),
            transform: Transform::default(),
            state: RasterState::default(),
            stats: RasterStats::default(),
        }
    }

    pub fn draw_triangle(&mut self, vertices: [Vec4; 3], framebuffer: &mut Framebuffer) -> GPUResult<()> {
        self.stats.triangles_submitted += 1;

        // Transform vertices
        let transformed = self.transform_vertices(vertices);
        
        // Clip against view frustum
        if !self.clip_triangle(&transformed) {
            self.stats.triangles_culled += 1;
            return Ok(());
        }

        // Perform backface culling
        if self.should_cull(&transformed) {
            self.stats.triangles_culled += 1;
            return Ok(());
        }

        // Rasterize triangle
        match self.state.polygon_mode {
            PolygonMode::Point => self.rasterize_points(&transformed, framebuffer),
            PolygonMode::Line => self.rasterize_wireframe(&transformed, framebuffer),
            PolygonMode::Fill => self.rasterize_filled(&transformed, framebuffer),
        }
    }

    fn transform_vertices(&self, vertices: [Vec4; 3]) -> [Vec4; 3] {
        let mvp = self.transform.projection
            .multiply(&self.transform.view)
            .multiply(&self.transform.model);

        [
            mvp.transform_vector(&vertices[0]),
            mvp.transform_vector(&vertices[1]),
            mvp.transform_vector(&vertices[2]),
        ]
    }

    fn rasterize_filled(&mut self, vertices: &[Vec4; 3], framebuffer: &mut Framebuffer) -> GPUResult<()> {
        // Convert to screen space
        let screen_verts = self.to_screen_space(vertices);

        // Calculate bounding box
        let (min_x, min_y, max_x, max_y) = self.get_bounding_box(&screen_verts);

        // Rasterize pixels
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(bary) = self.compute_barycentric(x as f32, y as f32, &screen_verts) {
                    if self.is_inside_triangle(&bary) {
                        self.stats.fragments_generated += 1;
                        
                        let depth = self.interpolate_depth(&vertices, &bary);
                        if self.depth_test(x, y, depth) {
                            self.write_pixel(x, y, depth, framebuffer)?;
                            self.stats.fragments_passed += 1;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn depth_test(&self, x: i32, y: i32, depth: f32) -> bool {
        if !self.state.depth_test {
            return true;
        }

        let idx = (y * self.viewport.width as i32 + x) as usize;
        depth <= self.depth_buffer[idx]
    }

    fn write_pixel(&mut self, x: i32, y: i32, depth: f32, framebuffer: &mut Framebuffer) -> GPUResult<()> {
        let idx = (y * self.viewport.width as i32 + x) as usize;
        
        if self.state.depth_write {
            self.depth_buffer[idx] = depth;
        }

        // Write color (simplified - would need actual color interpolation)
        framebuffer.write_pixel(x as u32, y as u32, [255, 255, 255, 255])
    }

    // Helper methods...
}

impl Viewport {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            min_depth: 0.0,
            max_depth: 1.0,
        }
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            model: Mat4::identity(),
            view: Mat4::identity(),
            projection: Mat4::identity(),
            viewport: Mat4::identity(),
        }
    }
}
