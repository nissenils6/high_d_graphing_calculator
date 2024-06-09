use sdl2::{video::{GLContext, GLProfile, SwapInterval, Window}, EventPump, Sdl};

pub struct Winsdl {
    pub sdl: Sdl,
    pub window: Window,
    pub gl_context: GLContext,
    pub event_pump: EventPump,
}

impl Winsdl {
    pub fn new(width: usize, height: usize, title: &str) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video_subsystem = sdl.video()?;

        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(GLProfile::Core);
        gl_attr.set_context_version(3, 3);

        let window = video_subsystem
            .window(title, width as u32, height as u32)
            .resizable()
            .opengl()
            .build()
            .map_err(|e| e.to_string())?;

        let gl_context = window.gl_create_context()?;
        gl::load_with(|s| {
            video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void
        });

        window.subsystem().gl_set_swap_interval(SwapInterval::VSync)?;

        let event_pump = sdl.event_pump()?;

        Ok(Winsdl {
            sdl, window, gl_context, event_pump
        })
    }
}
