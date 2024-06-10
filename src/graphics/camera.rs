use std::f32::consts::PI;

#[derive(Clone, Copy)]
pub struct Camera {
    pub focus: glm::Vec3,
    pub distance: f32,
    pub vertical_angle: f32,
    pub horizontal_angle: f32,
    pub screen_size: glm::Vec2,
    pub fov: f32,
    pub near_clip: f32,
    pub far_clip: f32,
}

impl Camera {
    pub fn new() -> Self {
        Camera {
            focus: glm::Vec3::zeros(),
            distance: 2.0,
            vertical_angle: PI / 4.0,
            horizontal_angle: 0.0,
            screen_size: glm::Vec2::new(800.0, 600.0),
            fov: PI / 2.0,
            near_clip: 0.2,
            far_clip: 100.0,
        }
    }

    pub fn matrix(&self) -> glm::Mat4 {
        let position = glm::Vec3::z() * -self.distance;
        let position = glm::rotate_x_vec3(&position, -self.vertical_angle);
        let position = glm::rotate_y_vec3(&position, self.horizontal_angle);
        let position = position + self.focus;

        let translation = glm::translation(&position);

        let vertical_rotation = glm::rotation(self.vertical_angle, &glm::Vec3::x());
        let horizontal_rotation = glm::rotation(-self.horizontal_angle, &glm::Vec3::y());
        
        let perspective = glm::perspective(self.screen_size.x / self.screen_size.y, self.fov, self.near_clip, self.far_clip);

        perspective * vertical_rotation * horizontal_rotation * translation
    }
}
