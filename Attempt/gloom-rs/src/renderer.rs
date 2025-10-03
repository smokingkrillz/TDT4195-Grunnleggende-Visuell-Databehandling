use crate::camera::Camera;
use crate::graphics;
use crate::scene::Scene;
use crate::shader;
use crate::mesh::{Terrain, Mesh};

pub struct Renderer {
    pub vao: u32,
    pub terrain: Mesh,
    pub shader_program: shader::Shader,
    pub alpha_location: i32,
    pub transform_matrix_location: i32,
}
// Renderer 
impl Renderer {
    pub unsafe fn new(_scene: &Scene) -> Self {
        let terrain = Terrain::load("resources/lunarsurface.obj");

        let vao = graphics::create_vao(&terrain.vertices, &terrain.indices, &terrain.colors, &terrain.normals);

        let shader_program = shader::ShaderBuilder::new()
            .attach_file("shaders/simple.vert")
            .attach_file("shaders/simple.frag")
            .link();

        shader_program.activate();

        // Get uniform locations
        let alpha_location = {
            let mut current_program: i32 = 0;
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut current_program);
            gl::GetUniformLocation(
                current_program as u32,
                b"uAlpha\0".as_ptr() as *const _,
            )
        };

        let transform_matrix_location = {
            let mut current_program: i32 = 0;
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut current_program);
            gl::GetUniformLocation(
                current_program as u32,
                b"uTransformMatrix\0".as_ptr() as *const _,
            )
        };

        Renderer {
            vao,
            terrain,
            shader_program,
            alpha_location,
            transform_matrix_location,
        }
    }

    pub unsafe fn render(&self, camera: &Camera) {
        // Clear the color and depth buffers
        gl::ClearColor(0.035, 0.046, 0.078, 1.0); // night sky
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        // Activate shader program before drawing
        self.shader_program.activate();
        // cull face is disabled to see all triangles
        gl::Disable(gl::CULL_FACE);
        gl::DepthMask(gl::TRUE);
        gl::Uniform1f(self.alpha_location, 0.9);

        gl::BindVertexArray(self.vao);

        // Get the transformation matrix from camera
        let transform_matrix = camera.get_scene_transform();
        
        // Apply the transformation to all triangles
        gl::UniformMatrix4fv(
            self.transform_matrix_location,
            1,
            gl::FALSE,
            transform_matrix.as_ptr(),
        );

        // Draw all triangles at once
        gl::DrawElements(
            gl::TRIANGLES,
            //terrain index count
            self.terrain.index_count,
            gl::UNSIGNED_INT,
            std::ptr::null(),
        );
    }
}
