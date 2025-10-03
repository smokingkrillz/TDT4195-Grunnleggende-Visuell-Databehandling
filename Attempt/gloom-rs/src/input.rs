use glutin::event::VirtualKeyCode;
use crate::camera::Camera;

/// InputHandler manages all keyboard input for camera movement and other controls
/// This struct handles the translation between keyboard events and camera movements,
/// implementing a tripod-style camera system with 6 degrees of freedom:
/// - 3 translation axes (X, Y, Z)
/// - 3 rotation axes (Yaw, Pitch, Roll)
pub struct InputHandler {
    /// Legacy field for testing - can be removed in final version
    pub arbitrary_number: f32,
}

impl InputHandler {
    /// Creates a new InputHandler with default values
    pub fn new() -> Self {
        InputHandler {
            arbitrary_number: 0.0,
        }
    }


    /// All movements are scaled by delta_time to ensure consistent speed regardless of framerate.
    pub fn handle_keyboard_input(&mut self, keys: &[VirtualKeyCode], camera: &mut Camera, delta_time: f32) {
        // Movement speeds (units per second)
        // These values can be adjusted to make camera movement faster or slower
        let move_speed = 50.0;        // Translation speed (world units per second)
        let rotation_speed = 3.0;    // Rotation speed (radians per second)

        // Process each currently pressed key
        for key in keys.iter() {
            match key {
            
                // Forward/Backward movement (Z-axis)
                VirtualKeyCode::W => {
                    //  forward (negative Z)
                    camera.translate(0.0, 0.0, -move_speed * delta_time);
                }
                VirtualKeyCode::S => {
                    //  backward (positive Z)
                    camera.translate(0.0, 0.0, move_speed * delta_time);
                }
                
                // Left/Right strafing (X-axis)
                VirtualKeyCode::A => {
                    //  left (negative X)
                    camera.translate(-move_speed * delta_time, 0.0, 0.0);
                }
                VirtualKeyCode::D => {
                    //  right (positive X)
                    camera.translate(move_speed * delta_time, 0.0, 0.0);
                }
                
                
                VirtualKeyCode::Space => {
                    // Move up (positive Y in world coordinates)
                    camera.translate(0.0, move_speed * delta_time, 0.0);
                }
                VirtualKeyCode::LShift => {
                   
                    camera.translate(0.0, -move_speed * delta_time, 0.0);
                }
                
                // These change the camera's orientation/viewing direction
                
                // Horizontal rotation (Yaw )
                VirtualKeyCode::Left => {
                    // Turn left (negative yaw rotation around Y-axis)
                    camera.rotate(-rotation_speed * delta_time, 0.0, 0.0);
                }
                VirtualKeyCode::Right => {
                    // Turn right (positive yaw rotation around Y-axis)
                    camera.rotate(rotation_speed * delta_time, 0.0, 0.0);
                }
                
                // Vertical rotation (Pitch)
                VirtualKeyCode::Up => {
                    // Look up (negative pitch rotation around X-axis)
                    camera.rotate(0.0, -rotation_speed * delta_time, 0.0);
                }
                VirtualKeyCode::Down => {
                    // Look down (positive pitch rotation around X-axis)
                    camera.rotate(0.0, rotation_speed * delta_time, 0.0);
                }
                
                // Roll rotation (tilt))
                VirtualKeyCode::Q => {
                    // Roll left (negative roll rotation around Z-axis)
                    camera.rotate(0.0, 0.0, -rotation_speed * delta_time);
                }
                VirtualKeyCode::E => {
                    // Roll right (positive roll rotation around Z-axis)
                    camera.rotate(0.0, 0.0, rotation_speed * delta_time);
                }
                
                
                // Default case: ignore unhandled keys
                _ => {}
            }
        }
    }
}
