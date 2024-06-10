use std::f32::consts::{PI, TAU};

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode,
    sys::XBufferOverflow,
};

#[derive(Clone, Copy)]
pub struct Camera {
    focus: glm::Vec3,
    distance: f32,
    vertical_angle: f32,
    horizontal_angle: f32,
    screen_size: glm::Vec2,
    fov: f32,
    near_clip: f32,
    far_clip: f32,
    movement_buttons_down: u8,
}

const MOVEMENT: [glm::Vec3; 6] = [
    glm::Vec3::new(0.0, 0.0, 1.0),
    glm::Vec3::new(1.0, 0.0, 0.0),
    glm::Vec3::new(0.0, 0.0, -1.0),
    glm::Vec3::new(-1.0, 0.0, 0.0),
    glm::Vec3::new(0.0, -1.0, 0.0),
    glm::Vec3::new(0.0, 1.0, 0.0),
];

fn movement_button(keycode: Keycode) -> Option<u8> {
    match keycode {
        Keycode::W => Some(0),
        Keycode::A => Some(1),
        Keycode::S => Some(2),
        Keycode::D => Some(3),
        Keycode::E => Some(4),
        Keycode::Q => Some(5),
        _ => None,
    }
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
            movement_buttons_down: 0,
        }
    }

    pub fn process_event(&mut self, event: &Event) {
        match event {
            &Event::KeyDown { keycode: Some(keycode), .. } => {
                if let Some(movement_button) = movement_button(keycode) {
                    self.movement_buttons_down |= 1 << movement_button;
                }
            }
            &Event::KeyUp { keycode: Some(keycode), .. } => {
                if let Some(movement_button) = movement_button(keycode) {
                    self.movement_buttons_down &= !(1 << movement_button);
                }
            }
            &Event::MouseWheel { y, .. } => {
                self.distance = (self.distance * 0.9f32.powf(y as f32)).clamp(0.2, 100.0);
            }
            &Event::MouseMotion { mousestate, xrel, yrel, .. } => {
                if mousestate.left() {
                    self.horizontal_angle = (self.horizontal_angle - xrel as f32 * TAU / 800.0) % TAU;
                    self.vertical_angle = (self.vertical_angle + yrel as f32 * TAU / 600.0).clamp(0.0, TAU / 4.0);
                }
            }
            &Event::Window { win_event: WindowEvent::Resized(width, height), .. } => {
                unsafe {
                    gl::Viewport(0, 0, width, height);
                }
                self.screen_size = glm::Vec2::new(width as f32, height as f32);
            }
            _ => {}
        }
    }

    pub fn tick(&mut self) {
        let mut movement = glm::Vec3::zeros();
        for i in 0..6 {
            if ((self.movement_buttons_down >> i) & 1) == 1 {
                movement += MOVEMENT[i];
            }
        }
        self.focus += glm::rotate_y_vec3(&(movement * self.distance * 0.02), self.horizontal_angle);
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
