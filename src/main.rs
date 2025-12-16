mod physics;

use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("Black Hole Renderer")
        .build(&event_loop)
        .unwrap();

    pollster::block_on(run(event_loop, window));
}

async fn run(event_loop: EventLoop<()>, window: winit::window::Window) {
    let instance = wgpu::Instance::default();
    let surface = instance.create_surface(&window).unwrap();

    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .unwrap();

    println!("Using GPU: {}", adapter.get_info().name);

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor::default(), None)
        .await
        .unwrap();

    let size = window.inner_size();

    // =======================
    // OUTPUT TEXTURE
    // =======================
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Output Texture"),
        size: wgpu::Extent3d {
            width: size.width,
            height: size.height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba16Float,
        usage: wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    });

    let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());

    // =======================
    // SHADER
    // =======================
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Compute Shader"),
        source: wgpu::ShaderSource::Wgsl(
            include_str!("../shaders/trace.wgsl").into(),
        ),
    });

    // =======================
    // BIND GROUP LAYOUT
    // =======================
    let bind_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Compute BGL"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::COMPUTE,
                ty: wgpu::BindingType::StorageTexture {
                    access: wgpu::StorageTextureAccess::WriteOnly,
                    format: wgpu::TextureFormat::Rgba16Float,
                    view_dimension: wgpu::TextureViewDimension::D2,
                },
                count: None,
            }],
        });

    let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("Compute BG"),
        layout: &bind_group_layout,
        entries: &[wgpu::BindGroupEntry {
            binding: 0,
            resource: wgpu::BindingResource::TextureView(&texture_view),
        }],
    });

    // =======================
    // PIPELINE
    // =======================
    let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Compute Pipeline Layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

    let compute_pipeline =
        device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&pipeline_layout),
            module: &shader,
            entry_point: "main",
        });

    //Rendring and compute of Textures 
    let surface_caps=surface.get_capabilities(&adapter);
    let surface_format=surface_caps.formats[0];

    let mut surface_config=wgpu::SurfaceConfiguration{
        usage:wgpu::TextureUsages::RENDER_ATTACHMENT,
        format:surface_format,
        width:size.width,
        height:size.height,
        present_mode:wgpu::PresentMode::Fifo,
        alpha_mode:surface_caps.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &surface_config);

    //rendering pipeline 
    let sampler=device.create_sampler(&wgpu::SamplerDescriptor::default());

    let render_bgl=device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
        label:Some("Render BGL"),
        entries:&[
            wgpu::BindGroupLayoutEntry{
                binding:0,
                visibility:wgpu::ShaderStages::FRAGMENT,
                ty:wgpu::BindingType::Texture { multisampled:false, view_dimension: wgpu::TextureViewDimension::D2,sample_type:wgpu::TextureSampleType::Float { filterable: true },
                },
                count:None,
            },
            wgpu::BindGroupLayoutEntry{
                binding:1,
                visibility:wgpu::ShaderStages::FRAGMENT,
                ty:wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count:None,
            },

        ],
    });

    let render_bg=device.create_bind_group(&wgpu::BindGroupDescriptor{
        label:Some("Render BG"),
        layout:&render_bgl,
        entries:&[
            wgpu::BindGroupEntry{
                binding:0,
                resource:wgpu::BindingResource::TextureView(&texture_view),
            },
            wgpu::BindGroupEntry{
                binding:1,
                resource:wgpu::BindingResource::Sampler(&sampler),
            },
        ],
    });

    let render_shader=device.create_shader_module(wgpu::ShaderModuleDescriptor{
        label:Some("Fullscreen Shader"),
        source:wgpu::ShaderSource::Wgsl(
            include_str!("../shaders/fullscreen.wgsl").into(),
        ),
    });

    let render_pipeline_layout=
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor{
            label:Some("Render Pipeline Layout"),
            bind_group_layouts:&[&render_bgl],
            push_constant_ranges:&[],
        });

    let render_pipeline=device.create_render_pipeline(&wgpu::RenderPipelineDescriptor{
        label:Some("Render Pipeline"),
        layout:Some(&render_pipeline_layout),
        vertex:wgpu::VertexState{
            module:&render_shader,
            entry_point:"vs_main",
            buffers:&[],
        },
        fragment:Some(wgpu::FragmentState { module: &render_shader, entry_point: "fs_main", targets: &[Some(wgpu::ColorTargetState{
            format:surface_format,
            blend:Some(wgpu::BlendState::REPLACE),
            write_mask:wgpu::ColorWrites::ALL,
        })],
        }),
        primitive:wgpu::PrimitiveState::default(),
        depth_stencil:None,
        multisample:wgpu::MultisampleState::default(),
        multiview:None,
    });

    // Request ONE redraw
    window.request_redraw();

    // =======================
    // EVENT LOOP
    // =======================
    event_loop.run(move |event, target| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                target.exit();
            }

            Event::WindowEvent { event:WindowEvent::RedrawRequested,window_id:_,} => {
                // Get the current frame first
                let frame = surface.get_current_texture().unwrap();
                let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

                // Compute pass
                let mut compute_encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Compute Encoder"),
                    });

                {
                    let mut pass =
                        compute_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                            label: Some("Compute Pass"),
                            timestamp_writes:None,
                        });

                    pass.set_pipeline(&compute_pipeline);
                    pass.set_bind_group(0, &bind_group, &[]);
                    let gx = (size.width + 7) / 8;
                    let gy = (size.height + 7) / 8;
                    pass.dispatch_workgroups(gx, gy, 1);
                }

                queue.submit(Some(compute_encoder.finish()));

                // Render pass
                let mut render_encoder =
                    device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                        label: Some("Render Encoder"),
                    });

                {
                    let mut rpass = render_encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                        label: Some("Render Pass"),
                        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                            view: &view,
                            resolve_target: None,
                            ops: wgpu::Operations {
                                load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                                store: wgpu::StoreOp::Store,
                            },
                        })],
                        depth_stencil_attachment: None,
                        timestamp_writes: None,
                        occlusion_query_set: None,
                    });

                    rpass.set_pipeline(&render_pipeline);
                    rpass.set_bind_group(0, &render_bg, &[]);
                    rpass.draw(0..3, 0..1);
                }

                queue.submit(Some(render_encoder.finish()));
                frame.present();
            }

            _ => {}
        }
    });
}

