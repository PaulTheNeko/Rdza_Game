extern crate wgpu;
extern crate winit;
extern crate raw_window_handle;

/* #[allow(unused)] // Na jakby co bo w przykładach było
pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0,
    0.0, -1.0, 0.0, 0.0,
    0.0, 0.0, 0.5, 0.0,
    0.0, 0.0, 0.5, 1.0,
); */


fn main() {
    const TITLE: &str = "Piersze testy";

    let event_loop = winit::event_loop::EventLoop::new();

    let (_window, instance, size, surface) = {
        use raw_window_handle::HasRawWindowHandle as _;


        let window = winit::window::Window::new(&event_loop).unwrap();
        window.set_title(TITLE);
        let hidpi_factor = window.hidpi_factor();
        let size = window.inner_size().to_physical(hidpi_factor);

        let instance = wgpu::Instance::new();
        let surface = instance.create_surface(window.raw_window_handle());

        (window, instance, size, surface)
    };
    
    let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::LowPower,
    });

    #[allow(unused)] // Jeszcze nie używamy
    let mut device = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    // Ładowanie shader'ów
    let vshade = include_bytes!("shader.vert.spv");
    let vshade_module = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&vshade[..])).unwrap());

    let fshade = include_bytes!("shader.frag.spv");
    let fshade_module = device.create_shader_module(&wgpu::read_spirv(std::io::Cursor::new(&fshade[..])).unwrap());

// Ukradzione!
    let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        bindings: &[],
    });
    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &bind_group_layout,
        bindings: &[],
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        bind_group_layouts: &[&bind_group_layout],
    });

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        layout: &pipeline_layout,
        vertex_stage: wgpu::ProgrammableStageDescriptor {
            module: &vshade_module,
            entry_point: "main",
        },
        fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
            module: &fshade_module,
            entry_point: "main",
        }),
        rasterization_state: Some(wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: wgpu::CullMode::None,
            depth_bias: 0,
            depth_bias_slope_scale: 0.0,
            depth_bias_clamp: 0.0,
        }),
        primitive_topology: wgpu::PrimitiveTopology::TriangleList,
        color_states: &[wgpu::ColorStateDescriptor {
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            color_blend: wgpu::BlendDescriptor::REPLACE,
            alpha_blend: wgpu::BlendDescriptor::REPLACE,
            write_mask: wgpu::ColorWrite::ALL,
        }],
        depth_stencil_state: None,
        index_format: wgpu::IndexFormat::Uint16,
        vertex_buffers: &[],
        sample_count: 1,
        sample_mask: !0,
        alpha_to_coverage_enabled: false,
    });
// Koniec Ukradzione

    let mut swap_chain = device.create_swap_chain(
        &surface,
        &wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width.round() as u32,
            height: size.height.round() as u32,
            present_mode: wgpu::PresentMode::Vsync,
        },
    );

}