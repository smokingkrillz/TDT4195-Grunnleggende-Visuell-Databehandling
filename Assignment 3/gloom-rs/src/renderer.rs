use crate::camera::Camera;
use crate::graphics;
use crate::mesh::{Helicopter, Terrain};
use crate::scene::Scene;
use crate::scene_graph::{Node, SceneNode};
use crate::shader;
use crate::toolbox;

// this is needed for dereferencing raw pointers in the scene graph

extern crate nalgebra_glm as glm;

#[inline]
fn node_ref(n: &Node) -> &SceneNode {
    unsafe { &**n }
}
#[inline]
fn node_mut(n: &mut Node) -> &mut SceneNode {
    unsafe { &mut ***n }
}

pub struct Renderer {
    pub root_node: Node,
    pub helicopters: Vec<Node>, 
    // Shader + uniforms
    pub shader_program: shader::Shader,
    pub alpha_location: i32,
    pub mvp_matrix_location: i32,
    pub model_matrix_location: i32,
}

impl Renderer {
    pub unsafe fn new(_scene: &Scene) -> Self {
    
        let helicopter_model = Helicopter::load("resources/helicopter.obj");
        let terrain = Terrain::load("resources/lunarsurface.obj");

        // creating terrain vao
        let terrain_vao = graphics::create_vao(
            &terrain.vertices,
            &terrain.indices,
            &terrain.colors,
            &terrain.normals,
        );

        // creating helicopter vaos only once
        let body_vao = graphics::create_vao(
            &helicopter_model.body.vertices,
            &helicopter_model.body.indices,
            &helicopter_model.body.colors,
            &helicopter_model.body.normals,
        );
        let door_vao = graphics::create_vao(
            &helicopter_model.door.vertices,
            &helicopter_model.door.indices,
            &helicopter_model.door.colors,
            &helicopter_model.door.normals,
        );
        let main_vao = graphics::create_vao(
            &helicopter_model.main_rotor.vertices,
            &helicopter_model.main_rotor.indices,
            &helicopter_model.main_rotor.colors,
            &helicopter_model.main_rotor.normals,
        );
        let tail_vao = graphics::create_vao(
            &helicopter_model.tail_rotor.vertices,
            &helicopter_model.tail_rotor.indices,
            &helicopter_model.tail_rotor.colors,
            &helicopter_model.tail_rotor.normals,
        );

        let shader_program = shader::ShaderBuilder::new()
            .attach_file("shaders/simple.vert")
            .attach_file("shaders/simple.frag")
            .link();

        shader_program.activate();

        
        let alpha_location = {
            let mut current_program: i32 = 0;
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut current_program);
            gl::GetUniformLocation(current_program as u32, b"uAlpha\0".as_ptr() as *const _)
        };

        let mvp_matrix_location = {
            let mut current_program: i32 = 0;
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut current_program);
            gl::GetUniformLocation(
                current_program as u32,
                b"uMVPMatrix\0".as_ptr() as *const _,
            )
        };

        let model_matrix_location = {
            let mut current_program: i32 = 0;
            gl::GetIntegerv(gl::CURRENT_PROGRAM, &mut current_program);
            gl::GetUniformLocation(
                current_program as u32,
                b"uModelMatrix\0".as_ptr() as *const _,
            )
        };

      
        let mut terrain_node = SceneNode::from_vao(terrain_vao, terrain.index_count);

        // Create 5 helicopters that reference the shared VAOs
        let mut helicopters: Vec<Node> = Vec::new();
        for i in 0..5 {
            
            let mut body_node = SceneNode::from_vao(body_vao, helicopter_model.body.index_count);
            let mut door_node = SceneNode::from_vao(door_vao, helicopter_model.door.index_count);
            let mut main_node = SceneNode::from_vao(main_vao, helicopter_model.main_rotor.index_count);
            let mut tail_node = SceneNode::from_vao(tail_vao, helicopter_model.tail_rotor.index_count);


            // Tail: given by assignment
            node_mut(&mut tail_node).reference_point = glm::vec3(0.35, 2.3, 10.4);
            
            node_mut(&mut door_node).reference_point = glm::vec3(-1.0, 0.0, 0.0);
      

            // Helicopter root (group node, not drawable)
            let mut heli_root = SceneNode::new();

            // Hook up parts under the helicopter root
            node_mut(&mut heli_root).add_child(node_ref(&body_node));
            node_mut(&mut heli_root).add_child(node_ref(&door_node));
            node_mut(&mut heli_root).add_child(node_ref(&main_node));
            node_mut(&mut heli_root).add_child(node_ref(&tail_node));

            // Spread them out a bit initially to avoid overlap before animation kicks in
            node_mut(&mut heli_root).position = glm::vec3(i as f32 * 50.0, 20.0, 0.0);

            // Add a test rotation to the first helicopter to verify transformations work
            if i == 0 {
                node_mut(&mut heli_root).rotation.y = std::f32::consts::PI / 4.0; // 45 degrees
            }
            
            helicopters.push(heli_root);
        }

        // add helicopters under terrain
        for heli_root in &helicopters {
            node_mut(&mut terrain_node).add_child(node_ref(heli_root));
        }

        // Create root and add terrain
        let mut root_node = SceneNode::new();
        node_mut(&mut root_node).add_child(node_ref(&terrain_node));

        println!("Scene Graph ready. Terrain + {} helicopters.", helicopters.len());

        Renderer {
            root_node,
            helicopters,
            shader_program,
            alpha_location,
            mvp_matrix_location,
            model_matrix_location
        }
    }

    pub unsafe fn render(&self, camera: &Camera) {
        // Clear
        gl::ClearColor(0.035, 0.046, 0.078, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

        // Shader and global state
        self.shader_program.activate();
        gl::Enable(gl::CULL_FACE);
        gl::DepthMask(gl::TRUE);
        gl::Uniform1f(self.alpha_location, 0.9);

        // VP and identity
        let vp = camera.get_view_projection_matrix();
        let identity: glm::Mat4 = glm::identity();

        // Traverse and draw
        Self::draw_scene(
            node_ref(&self.root_node),
            &vp,
            &identity,
            self.mvp_matrix_location,
            self.model_matrix_location,
        );
    }


    pub fn update_animations(&mut self, elapsed: f32) {
        let main_rotor_speed = 5_000.0;
        let tail_rotor_speed = 5_000.0;

        for (i, heli_node) in self.helicopters.iter_mut().enumerate() {
            let heli = node_mut(heli_node);

            // Offset each helicopter along the same path to avoid collisions
            let offset = i as f32 * 0.75;
            let heading = toolbox::simple_heading_animation(elapsed + offset);

           // Update helicopter position to move forward
            heli.position = glm::vec3(heading.x, 20.0, heading.z);
            heli.rotation = glm::vec3(heading.pitch, heading.yaw, heading.roll);

            // Children order: 0=body, 1=door, 2=main rotor, 3=tail rotor
            if heli.n_children() >= 4 {
                let main_rotor = heli.get_child(2);
                main_rotor.rotation = glm::vec3(0.0, 1.0, 0.0) * main_rotor_speed * elapsed;

                let tail_rotor = heli.get_child(3);
                tail_rotor.rotation = glm::vec3(1.0, 0.0, 0.0) * tail_rotor_speed * elapsed;
            }
        }

    }

    /// Recursive scene traversal + draw
    unsafe fn draw_scene(
        node: &SceneNode,
        view_projection_matrix: &glm::Mat4,
        parent: &glm::Mat4,
        mvp_matrix_location: i32,
        model_matrix_location: i32,
    ) {

        let x = glm::rotation(node.rotation.x, &glm::vec3(1.0, 0.0, 0.0));
        let y = glm::rotation(node.rotation.y, &glm::vec3(0.0, 1.0, 0.0));
        let z = glm::rotation(node.rotation.z, &glm::vec3(0.0, 0.0, 1.0));
        let rot = z * x * y;

        let scl = glm::scaling(&node.scale);
        let t_pos = glm::translation(&node.position);
        let t_to_pivot = glm::translation(&node.reference_point);
        let t_from_pivot = glm::translation(&(-node.reference_point));

        let local = t_pos * t_to_pivot * rot * scl * t_from_pivot;


        let world = parent * local;

        // Draw if this node is drawable
        if node.vao_id != 0 && node.index_count > 0 {
            gl::BindVertexArray(node.vao_id);

            let mvp = view_projection_matrix * world;
            gl::UniformMatrix4fv(mvp_matrix_location, 1, gl::FALSE, mvp.as_ptr());
            gl::UniformMatrix4fv(model_matrix_location, 1, gl::FALSE, world.as_ptr());

            gl::DrawElements(
                gl::TRIANGLES,
                node.index_count,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }

        // Recurse
        for &child in &node.children {
            Self::draw_scene(
                unsafe { &*child },
                view_projection_matrix,
                &world,
                mvp_matrix_location,
                model_matrix_location,
            );
        }
    }
}
