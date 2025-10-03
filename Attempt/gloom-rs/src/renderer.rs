use crate::camera::Camera;
use crate::graphics;
use crate::scene::Scene;
use crate::shader;
use crate::mesh::{Terrain, Mesh, Helicopter};
use crate::scene_graph::{SceneNode, Node};

pub struct Renderer {
    pub root_node: Node,
    pub shader_program: shader::Shader,
    pub alpha_location: i32,
    pub transform_matrix_location: i32,
}
// Renderer 
impl Renderer {


    pub unsafe fn new(_scene: &Scene) -> Self {
        // Load terrain
        let terrain = Terrain::load("resources/lunarsurface.obj");
        let terrain_vao = graphics::create_vao(&terrain.vertices, &terrain.indices, &terrain.colors, &terrain.normals);

        // Load helicopter
        let helicopter = Helicopter::load("resources/helicopter.obj");
        

        
        // Create separate VAOs for each helicopter part
        let helicopter_body_vao = graphics::create_vao(&helicopter.body.vertices, &helicopter.body.indices, &helicopter.body.colors, &helicopter.body.normals);
        let helicopter_door_vao = graphics::create_vao(&helicopter.door.vertices, &helicopter.door.indices, &helicopter.door.colors, &helicopter.door.normals);
        let helicopter_main_rotor_vao = graphics::create_vao(&helicopter.main_rotor.vertices, &helicopter.main_rotor.indices, &helicopter.main_rotor.colors, &helicopter.main_rotor.normals);
        let helicopter_tail_rotor_vao = graphics::create_vao(&helicopter.tail_rotor.vertices, &helicopter.tail_rotor.indices, &helicopter.tail_rotor.colors, &helicopter.tail_rotor.normals);

        

        let mut root_node = SceneNode::new();

        // 2) Create terrain node
        let mut terrain_node = SceneNode::from_vao(terrain_vao, terrain.index_count);
        terrain_node.position = glm::vec3(0.0, 0.0, -5.0);

        // 3) Create helicopter root node (positioned above terrain)
        let mut helicopter_root_node = SceneNode::new();
        helicopter_root_node.position = glm::vec3(0.0, 20.0, 0.0);

        // 4) Create individual helicopter part nodes with proper reference points
        let mut helicopter_body_node = SceneNode::from_vao(helicopter_body_vao, helicopter.body.index_count);
        
        let mut helicopter_door_node = SceneNode::from_vao(helicopter_door_vao, helicopter.door.index_count);
      
        
        let mut helicopter_main_rotor_node = SceneNode::from_vao(helicopter_main_rotor_vao, helicopter.main_rotor.index_count);
        
        let mut helicopter_tail_rotor_node = SceneNode::from_vao(helicopter_tail_rotor_vao, helicopter.tail_rotor.index_count);
        helicopter_tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4); // Tail rotor reference point given


        //testing 
        // helicopter_body_node.rotation.y = std::f32::consts::PI / 6.0; // 30 degrees around Y-axis
        // helicopter_body_node.position.x = 2.0; 
        // 5) Build the hierarchy: attach helicopter parts to helicopter root
        helicopter_root_node.add_child(&helicopter_body_node);
        helicopter_root_node.add_child(&helicopter_door_node);
        helicopter_root_node.add_child(&helicopter_main_rotor_node);
        helicopter_root_node.add_child(&helicopter_tail_rotor_node);

        // 6) Attach terrain and helicopter to scene root
        root_node.add_child(&terrain_node);
        root_node.add_child(&helicopter_root_node);

        // Print the scene graph for debugging
        println!("=== SCENE GRAPH STRUCTURE ===");
        root_node.print();

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
            root_node,
            shader_program,
            alpha_location,
            transform_matrix_location,
        }
    }

    pub unsafe fn render(&self, camera: &Camera, elapsed_time: f32) {
        // Clear the color and depth buffers
        gl::ClearColor(0.035, 0.046, 0.078, 1.0); 
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        // Activate shader program before drawing
        self.shader_program.activate();
        gl::Disable(gl::CULL_FACE);
        gl::DepthMask(gl::TRUE);
        gl::Uniform1f(self.alpha_location, 0.9);

        // Update rotor animations based on elapsed time
        self.update_animations(elapsed_time);

        // Get the View-Projection matrix from camera
        let view_projection_matrix = camera.get_view_projection_matrix();
        
        // Start with identity matrix for model transformations
        let identity_model = glm::Mat4::identity();
        
        // Traverse and draw the scene graph starting from root
        self.draw_scene(&*self.root_node, &view_projection_matrix, &identity_model);
    }

    unsafe fn update_animations(&self, elapsed_time: f32) {
        // Animation speeds 
        let rotor_speed = 8.0;
  
        
        // Calculate current rotation based on elapsed time
        let main_rotor_rotation = elapsed_time * rotor_speed;
        let tail_rotor_rotation = elapsed_time * rotor_speed;

        // Access the root node and navigate to helicopter parts using unsafe pointers
        let root = &*self.root_node;
        if root.children.len() >= 2 {
            // Get helicopter root node (should be second child after terrain)
            let helicopter_root_ptr = root.children[1];
            let helicopter_root = &mut *helicopter_root_ptr;
            
            if helicopter_root.children.len() >= 4 {
                // Main rotor should be third child 
                let main_rotor_ptr = helicopter_root.children[2];
                let main_rotor = &mut *main_rotor_ptr;
                main_rotor.rotation.y = main_rotor_rotation;
                
                // Tail rotor should be fourth child
                let tail_rotor_ptr = helicopter_root.children[3];
                let tail_rotor = &mut *tail_rotor_ptr;
                tail_rotor.rotation.x = tail_rotor_rotation;
            }
        }
    }

    unsafe fn draw_scene(&self, node: &SceneNode, view_projection_matrix: &glm::Mat4, transformation_so_far: &glm::Mat4) {
        // Step 1: Calculate individual transformation matrices for this node
        let translation_matrix = glm::translation(&node.position);
        let scale_matrix = glm::scaling(&node.scale);
        
        // Step 2: Handle rotation around reference point
        // CORRECT ORDER: Move to reference point, rotate, then move back
        let translate_to_reference = glm::translation(&(-node.reference_point));
        let translate_from_reference = glm::translation(&node.reference_point);
        
        // Step 3: Calculate rotation matrices (Euler angles: Z * Y * X order)
        let rotation_x = glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0));
        let rotation_y = glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0));
        let rotation_z = glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0));
        let combined_rotation = rotation_z * rotation_x * rotation_y;
        
        // Step 4: Combine this node's relative transformations  
        // CORRECT ORDER: Translate_to_ref * Rotate * Translate_from_ref
        let rotation_around_reference = translate_from_reference * combined_rotation * translate_to_reference;
        let model_matrix = translation_matrix * rotation_around_reference * scale_matrix;

        // Step 5: Combine with parent's accumulated model transformations
        let current_model_matrix = transformation_so_far * model_matrix;

        // Step 6: If this node has geometry to draw (index_count > 0)
        if node.index_count > 0 {
            gl::BindVertexArray(node.vao_id);
            
            // 
            //multiplication order is ViewProjection * Model
            let mvp_matrix = view_projection_matrix * current_model_matrix;
            
            // Apply the final MVP matrix to the vertex shader
            gl::UniformMatrix4fv(
                self.transform_matrix_location,
                1,
                gl::FALSE,
                mvp_matrix.as_ptr(),
            );

            // Draw the geometry
            gl::DrawElements(
                gl::TRIANGLES,
                node.index_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        // Step 8: Recursively traverse all children with accumulated model transformations
        for &child_ptr in &node.children {
            let child = &*child_ptr;
            self.draw_scene(child, view_projection_matrix, &current_model_matrix);
        }
    }
}
