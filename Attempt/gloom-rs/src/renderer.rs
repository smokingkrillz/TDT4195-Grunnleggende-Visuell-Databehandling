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
        helicopter_body_node.reference_point = glm::vec3(0.0, 0.0, 0.0); // Body rotates around its center
        
        let mut helicopter_door_node = SceneNode::from_vao(helicopter_door_vao, helicopter.door.index_count);
        helicopter_door_node.reference_point = glm::vec3(0.0, 0.0, 0.0); // Door rotates around its hinge
        
        let mut helicopter_main_rotor_node = SceneNode::from_vao(helicopter_main_rotor_vao, helicopter.main_rotor.index_count);
        helicopter_main_rotor_node.reference_point = glm::vec3(0.0, 0.0, 0.0); // Main rotor rotates around its center
        
        let mut helicopter_tail_rotor_node = SceneNode::from_vao(helicopter_tail_rotor_vao, helicopter.tail_rotor.index_count);
        helicopter_tail_rotor_node.reference_point = glm::vec3(0.35, 2.3, 10.4); // Tail rotor reference point as specified

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

    pub unsafe fn render(&self, camera: &Camera) {
        // Clear the color and depth buffers
        gl::ClearColor(0.035, 0.046, 0.078, 1.0); 
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        // Activate shader program before drawing
        self.shader_program.activate();
        gl::Disable(gl::CULL_FACE);
        gl::DepthMask(gl::TRUE);
        gl::Uniform1f(self.alpha_location, 0.9);

        // Get the base transformation matrix from camera
        let view_projection_matrix = camera.get_view_projection_matrix();
        
        // Traverse and draw the scene graph starting from root
        self.draw_scene(&*self.root_node, &view_projection_matrix);
    }

    unsafe fn draw_scene(&self, node: &SceneNode, parent_transform: &glm::Mat4) {
        // Calculate this node's transformation matrix
        let translation = glm::translation(&node.position);
        let scale = glm::scaling(&node.scale);
        
        // Reference point transformations for rotation
        let to_reference = glm::translation(&node.reference_point);
        let from_reference = glm::translation(&(-node.reference_point));
        
        // Rotation matrices
        let rotation_x = glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0));
        let rotation_y = glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0));
        let rotation_z = glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0));
        let rotation = rotation_z * rotation_y * rotation_x;
     
        // Combine transformations: Translation * (Translate_to_ref * Rotate * Translate_from_ref) * Scale
        let node_transform = translation * to_reference * rotation * from_reference * scale;
        let final_transform = parent_transform * node_transform;

        // If this node has geometry to draw (index_count > 0)
        if node.index_count > 0 {
            gl::BindVertexArray(node.vao_id);
            
            //  transformation matrix
            gl::UniformMatrix4fv(
                self.transform_matrix_location,
                1,
                gl::FALSE,
                final_transform.as_ptr(),
            );

            // Draw the geometry
            gl::DrawElements(
                gl::TRIANGLES,
                node.index_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        // Recursively draw all children
        for &child_ptr in &node.children {
            let child = &*child_ptr;
            self.draw_scene(child, &final_transform);
        }
    }
}
