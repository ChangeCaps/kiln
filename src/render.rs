use futures_lite::future;

pub struct Renderer {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub surface: wgpu::Surface,
}

impl Renderer {
    pub unsafe fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let surface = unsafe { instance.create_surface(window) };

        let adapter_fut = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        });

        let adapter = future::block_on(adapter_fut).unwrap();

        let device_fut = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("kiln-device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        );

        let (device, queue) = future::block_on(device_fut).unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        surface.configure(&device, &config);

        Self {
            device,
            queue,
            config,
            surface,
        }
    }

    pub fn configure(&self) {
        self.surface.configure(&self.device, &self.config);
    }
}
