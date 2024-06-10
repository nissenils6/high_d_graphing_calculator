extern crate nalgebra_glm as glm;

pub mod graphics;
pub mod math;

use math::expr::Expr;
use std::f32::consts::TAU;

use graphics::{camera::Camera, graphics3d::Graphics3D, winsdl::*};
use sdl2::event::Event;

fn f(x: f32, z: f32) -> f32 {
    0.25 * (x * x + z * z)
}

fn main() -> Result<(), String> {
    // font_test();

    let expr = Expr::add(Expr::mul(Expr::Constant(6.0), Expr::Constant(8.0)), Expr::Constant(4.0));

    println!("{}", expr);
    println!("{}", expr.eval());

    let mut sdl = Winsdl::new(800, 600, "My window")?;

    let mut graphics: Graphics3D = Graphics3D::new()?;

    let scale = TAU.sqrt() * 2.0;
    let steps = 250.0;
    graphics.surface(-scale, scale, scale / steps, -scale, scale, scale / steps, f);

    let mut camera = Camera::new();

    'running: loop {
        for event in sdl.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'running,
                e => camera.process_event(&e),
            }
        }

        camera.tick();
        graphics.render(&camera);

        sdl.window.gl_swap_window();
    }

    Ok(())
}
