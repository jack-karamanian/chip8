use std::{error::Error, str::FromStr};

use crate::graphics::{DISPLAY_HEIGHT, DISPLAY_WIDTH};

pub mod cpu;
pub mod graphics;
pub mod input;
pub mod instruction;
pub mod mmu;

#[derive(Debug)]
struct Chip8Error {
    message: String,
}
impl Chip8Error {
    pub fn new(message: &str) -> Chip8Error {
        Chip8Error {
            message: String::from_str(message).unwrap(),
        }
    }
}

impl Error for Chip8Error {}
impl std::fmt::Display for Chip8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = std::vec::Vec::from_iter(std::env::args());
    let rom_path = match args.get(1) {
        Some(rom_path) => rom_path,
        None => {
            println!("Rom path is required");
            return Err(Box::new(Chip8Error::new("Rom path is required")));
        }
    };

    let rom = match std::fs::read(rom_path) {
        Ok(data) => data,
        Err(e) => {
            println!("Failed to read rom: {}", e);
            return Err(Box::new(Chip8Error::new("Failed to read rom")));
        }
    };
    println!("Loaded ROM: {}", rom_path);
    let mut cpu = cpu::Cpu::new();
    let mut mmu = mmu::Mmu::new();

    mmu.load_rom(rom);

    let sdl_context = sdl2::init()?;
    let video = sdl_context.video()?;
    {
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(4, 6);
    }

    let window = video
        .window("chip8", 1920, 1080)
        .opengl()
        .position_centered()
        .build()?;

    let _gl_context = window.gl_create_context()?;
    gl::load_with(|s| video.gl_get_proc_address(s) as _);

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);

    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &window);
    let renderer =
        imgui_opengl_renderer::Renderer::new(&mut imgui, |s| video.gl_get_proc_address(s) as _);

    let mut graphics = graphics::Graphics::new();

    let texture = unsafe {
        use gl::types::GLuint;
        let mut gl_texture: GLuint = 0;

        let data = graphics.to_rgba();

        gl::GenTextures(1, std::ptr::addr_of_mut!(gl_texture));
        gl::BindTexture(gl::TEXTURE_2D, gl_texture);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);

        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGBA as i32,
            graphics::DISPLAY_WIDTH as i32,
            graphics::DISPLAY_HEIGHT as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            data.as_ptr() as *const std::ffi::c_void,
        );
        gl_texture
    };

    let mut events = sdl_context.event_pump()?;

    'quit: loop {
        for event in events.poll_iter() {
            imgui_sdl2.handle_event(&mut imgui, &event);

            if imgui_sdl2.ignore_event(&event) {
                continue;
            }

            match event {
                sdl2::event::Event::Quit { .. } => break 'quit,
                _ => {}
            }
        }

        unsafe {
            let data = graphics.to_rgba();

            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);

            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                graphics::DISPLAY_WIDTH as i32,
                graphics::DISPLAY_HEIGHT as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data.as_ptr() as *const std::ffi::c_void,
            );
        }

        imgui_sdl2.prepare_frame(imgui.io_mut(), &window, &events.mouse_state());

        let ui = imgui.frame();
        ui.show_demo_window(&mut true);
        ui.window("Test").build(|| {
            let texture_id = imgui::TextureId::new(texture as usize);
            if ui.button("Step") {
                cpu.step(&mut mmu, &mut graphics);
            }
            imgui::Image::new(
                texture_id,
                [(DISPLAY_WIDTH as f32) * 4.0, (DISPLAY_HEIGHT as f32) * 4.0],
            )
            .build(&ui)
        });

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        imgui_sdl2.prepare_render(&ui, &window);
        renderer.render(&mut imgui);

        window.gl_swap_window();
    }

    Ok(())
}
