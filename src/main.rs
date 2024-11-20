use camera::{Camera, CameraController};
use core::time;
use std::env;
use winit::{
    application::ApplicationHandler,
    event::{DeviceEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
};

mod camera;

mod model;
mod texture;

mod renderer;
use renderer::Renderer;

struct App<'a> {
    renderer: Option<Renderer<'a>>,
    camera: Camera,
    camera_controller: CameraController,
    current_frame_start: std::time::Instant,
    last_frame_duration: std::time::Duration,
}

impl ApplicationHandler for App<'_> {
    //used for initialization as well as resume
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.renderer.is_none() {
            self.renderer = Some(Renderer::new(event_loop, &mut self.camera))
        }
        self.renderer.as_mut().unwrap().window.request_redraw()
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: DeviceEvent,
    ) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.camera_controller.mouse_delta.0 += delta.0 as f32;
                self.camera_controller.mouse_delta.1 += delta.1 as f32;
            }
            _ => (),
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(new_physical_size) => {
                self.renderer.as_mut().unwrap().resize(new_physical_size, &mut self.camera)
            }
            WindowEvent::KeyboardInput {
                device_id: _,
                event,
                is_synthetic: _,
            } => {
                use winit::keyboard::{KeyCode, PhysicalKey};
                let is_pressed = event.state.is_pressed();
                match event.physical_key {
                    PhysicalKey::Code(KeyCode::KeyA) | PhysicalKey::Code(KeyCode::ArrowLeft) => {
                        self.camera_controller.left_pressed = is_pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyD) | PhysicalKey::Code(KeyCode::ArrowRight) => {
                        self.camera_controller.right_pressed = is_pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyS) | PhysicalKey::Code(KeyCode::ArrowDown) => {
                        self.camera_controller.backward_pressed = is_pressed;
                    }
                    PhysicalKey::Code(KeyCode::KeyW) | PhysicalKey::Code(KeyCode::ArrowUp) => {
                        self.camera_controller.forward_pressed = is_pressed;
                    }
                    _ => (),
                }
            }
            WindowEvent::RedrawRequested => {
                let static_objects_to_render;
                let dynamic_objects_to_render;
                let instanced_static_objects_to_render;

                self.renderer.as_mut().unwrap().render_pass(&self.camera);
                self.last_frame_duration = self.current_frame_start.elapsed();
                self.current_frame_start = std::time::Instant::now();
            }
            _ => (),
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut app = App {
        renderer: None,
        camera_controller: CameraController::new(0.1, 0.001),
        camera: Camera::new(1920.0 / 1080.0),
        current_frame_start: std::time::Instant::now(),
        last_frame_duration: time::Duration::default(),
    };
    let event_loop = EventLoop::new().expect("error creating event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    _ = event_loop.run_app(&mut app);
}
