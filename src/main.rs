use std::{env, sync::Arc};
use futures::executor::block_on;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

struct WGPUState<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    // window must outlive surface
    window: Arc<winit::window::Window>,
}

impl WGPUState<'_> {
    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}

struct App<'a> {
    wgpu_state: Option<WGPUState<'a>>,
}

impl ApplicationHandler for App<'_> {
    fn resumed(& mut self, event_loop: &ActiveEventLoop) {
        if self.wgpu_state.is_none() {
            self.wgpu_state = Some(WGPUState::new(event_loop));
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
            WindowEvent::Resized(new_physical_size) => self.wgpu_state.as_mut().unwrap().resize(new_physical_size),
            WindowEvent::RedrawRequested => {
                let wgpu_state = match self.wgpu_state.as_ref() {
                    Some(v) => v,
                    None => {
                        eprintln!("wgpu_state is null!");
                        return
                    }
                };
                let surface_texture = wgpu_state.surface.get_current_texture().expect("failed to get surface texture");
                let texture_view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                let mut command_encoder = wgpu_state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render encoder"),
                });
                _ = command_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &texture_view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color{
                                r: 0.1,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    depth_stencil_attachment: None,
                    occlusion_query_set: None,
                    timestamp_writes: None,
                });
                wgpu_state.queue.submit(std::iter::once(command_encoder.finish()));
                surface_texture.present();
            }
            _ => (),
        }
    }



}

impl<'a> WGPUState<'a> {
    fn new(event_loop: &ActiveEventLoop) -> Self {

        let window = Arc::new(event_loop.create_window(Window::default_attributes()).expect("error creating window"));

        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::PRIMARY,
            ..Default::default()
        });

        let surface = instance.create_surface(window.clone()).expect("error creating surface");

        let adapter_future = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        });
        let adapter = block_on(adapter_future).unwrap();

        let device_future = adapter.request_device(&wgpu::DeviceDescriptor {
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                label: None,
                memory_hints: wgpu::MemoryHints::Performance, 
            }, 
            None,
        );
        let (device, queue) = block_on(device_future).expect("error creating device");

        let surface_capabilities = surface.get_capabilities(&adapter);

        let surface_format = surface_capabilities.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_capabilities.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: surface_capabilities.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        Self {
            window: window.clone(),
            surface,
            device,
            queue,
            config: surface_config,
            size,
        }
    }
}

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let mut app = App {
        wgpu_state: None,
    };
    let event_loop = EventLoop::new().expect("error creating event loop");
    event_loop.set_control_flow(ControlFlow::Poll);
    _ = event_loop.run_app(&mut app);
}
