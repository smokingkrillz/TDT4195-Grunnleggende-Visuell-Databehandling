pub struct Scene {
    pub vertices: Vec<f32>,
    pub colors: Vec<f32>,
    pub indices: Vec<u32>,
}

impl Scene {
    pub fn new() -> Self {
        // 3 triangles. All overlap near (0,0). Each triangle's 3 vertices have the same z value.
        // z_far > z_mid > z_near added f32 to use memory efficiently
        let z_far = 0.30_f32; // draw first, blue
        let z_middle = 0.00_f32; // draw second, green
        let z_near = -0.30_f32; // draw last, red

        let vertices: Vec<f32> = vec![
            // Triangle A - Red triangle closest
            -0.6, -0.2, z_far,
            0.6, -0.2, z_far,
            0.0, 0.6, z_far,
            // Triangle B now at MIDDLE depth
            -0.6, 0.2, z_middle,
            0.6, 0.2, z_middle,
            0.0, -0.6, z_middle,
            // Triangle C now at FAR depth BLUE
            -0.2, -0.6, z_near,
            -0.2, 0.6, z_near,
            0.6, 0.0, z_near,
        ];

        // Each triangle has SAME color per vertex, DIFFERENT colors between triangles
        let colors: Vec<f32> = vec![
            // Triangle A:  RED  
            1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            // Triangle B:  GREEN  
            0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            // Triangle C:  BLUE 
            0.0, 0.0, 1.0, 0.0, 0.0, 1.0, 0.0, 0.0, 1.0,
        ];

        // CCW indices per triangle
        let indices: Vec<u32> = vec![
            0, 1, 2, // A (far)
            3, 4, 5, // B (mid)
            6, 7, 8, // C (near)
        ];

        Scene {
            vertices,
            colors,
            indices,
        }
    }
}
