#![windows_subsystem = "windows"]

use crate::gui::Gui;
use log::error;
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod gui;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("OpenEXR File Viewer")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

        Pixels::new(WIDTH, HEIGHT, surface_texture)?
    };

    // Set up Dear ImGui
    let mut gui = Gui::new(&window, &pixels);

    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        panic!("Expected an argument with OpenEXR image file path");
    }

    set_pixels_to_image(pixels.get_frame(), &args[1])?;

    event_loop.run(move |event, _, control_flow| {
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            // Prepare Dear ImGui
            gui.prepare(&window).expect("gui.prepare() failed");

            // Render everything together
            let render_result = pixels.render_with(|encoder, render_target, context| {
                // Render the world texture
                context.scaling_renderer.render(encoder, render_target);

                // Render Dear ImGui
                gui.render(&window, encoder, render_target, context)
                    .expect("gui.render() failed");
            });

            // Basic error handling
            if render_result
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        gui.handle_event(&window, &event);
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // Resize the window
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            window.request_redraw();
        }
    });
}

fn set_pixels_to_image(frame: &mut [u8], path: &str) -> anyhow::Result<()> {
    let exr_pixels = openexr_rs::read_image_pixels(path)?;

    // Draw the world
    draw_image(frame, exr_pixels);

    Ok(())
}

fn draw_image(frame: &mut [u8], _pixels: ()) {
    let color = [0x48, 0xb2, 0xe8, 0xff];

    for (_, pixel) in frame.chunks_exact_mut(4).enumerate() {
        pixel.copy_from_slice(&color);
    }
}
