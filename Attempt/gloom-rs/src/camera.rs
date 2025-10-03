extern crate nalgebra_glm as glm;

pub struct Camera {
    pub aspect_ratio: f32,
    pub fovy: f32,
    pub near: f32,
    pub far: f32,
    
    // Camera position (translation)
    pub x: f32,
    pub y: f32,
    pub z: f32,
    
    // Camera rotation
    pub yaw: f32,   // Y-axis
    pub pitch: f32, //  X-axis
    pub roll: f32,  //  Z-axis (tilt)
}

impl Camera {
    pub fn new(aspect_ratio: f32) -> Self {
        Camera {
            aspect_ratio,
            fovy: 1.2, 
            near: 1.0,
            far: 1000.0,
            
          
            x: 0.0,
            y: 200.0,
            z: -5.0,
            yaw: 0.0,
            pitch: -1.57,
            roll: 0.0,
        }
    }


    pub fn update_aspect_ratio(&mut self, width: u32, height: u32) {
        self.aspect_ratio = width as f32 / height as f32;
    }

    pub fn get_perspective_matrix(&self) -> glm::Mat4 {
        glm::perspective(self.aspect_ratio, self.fovy, self.near, self.far)
    }

    // Update camera position
    pub fn translate(&mut self, dx: f32, dy: f32, dz: f32) {
        self.x += dx;
        self.y += dy;
        self.z += dz;
    }

    // Update camera rotation
    pub fn rotate(&mut self, dyaw: f32, dpitch: f32, droll: f32) {
        self.yaw += dyaw;
        self.pitch += dpitch;
        self.roll += droll;
    }

    // Get the view matrix (inverse of camera transformation)
    pub fn get_view_matrix(&self) -> glm::Mat4 {
    
        // (B) translate
        let translation = glm::translate(&glm::Mat4::identity(), &glm::vec3(-self.x, -self.y, -self.z));
        // (A) rotation roll
        let rotation_z = glm::rotation(-self.roll, &glm::vec3(0.0, 0.0, 1.0));

        // (C) rotation yaw
        let rotation_y = glm::rotation(-self.yaw, &glm::vec3(0.0, 1.0, 0.0));
        
        // (D) rotation pitch
        let rotation_x = glm::rotation(-self.pitch, &glm::vec3(1.0, 0.0, 0.0));
        

        rotation_z * rotation_x * rotation_y * translation
    }


    pub fn get_view_projection_matrix(&self) -> glm::Mat4 {
        self.get_perspective_matrix() * self.get_view_matrix()
    }

    pub fn get_scene_transform(&self) -> glm::Mat4 {
    
        let model_matrix = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 0.0, -5.0));
        self.get_view_projection_matrix() * model_matrix
    }

    pub fn get_helicopter_transform(&self) -> glm::Mat4 {
        let model_matrix = glm::translate(&glm::Mat4::identity(), &glm::vec3(0.0, 20.0, -5.0));
        self.get_view_projection_matrix() * model_matrix
    }
}
