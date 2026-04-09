use std::sync::Arc;
use std::time::Instant;
use winit::dpi::PhysicalSize;
use wgpu::util::DeviceExt;
use wgpu::SurfaceError;
use winit::window::Window;

// Camera uniform — must match WGSL struct layout exactly (16-byte aligned vec3s)
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Camera {
    pub position:   [f32; 3],
    pub _pad1:      f32,
    pub forward:    [f32; 3],
    pub _pad2:      f32,
    pub right:      [f32; 3],
    pub _pad3:      f32,
    pub up:         [f32; 3],
    pub _pad4:      f32,
    pub resolution: [f32; 2],
    pub _pad5:      [f32; 2],   // pad to 80 bytes (multiple of 16)
}

impl Camera {
    pub fn new(width: u32, height: u32) -> Self {
        // Normalise forward so it's a proper unit vector
        let f = [0.0f32, -0.3, 1.0];
        let len = (f[0]*f[0] + f[1]*f[1] + f[2]*f[2]).sqrt();
        let forward = [f[0]/len, f[1]/len, f[2]/len];

        // right = cross(forward, world_up) then normalised
        let right = [forward[2], 0.0, -forward[0]];
        let rlen = (right[0]*right[0] + right[2]*right[2]).sqrt();
        let right = [right[0]/rlen, 0.0, right[2]/rlen];

        // up = cross(right, forward)
        let up = [
             right[2]*forward[1] - 0.0,
            -right[0]*forward[2] + right[2]*forward[0],
             right[0]*forward[1] - 0.0,
        ];

        Self {
            position:   [0.0, 2.0, -5.0],
            _pad1:      0.0,
            forward,
            _pad2:      0.0,
            right,
            _pad3:      0.0,
            up,
            _pad4:      0.0,
            resolution: [width as f32, height as f32],
            _pad5:      [0.0, 0.0],
        }
    }
}

pub struct State {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    pub queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    pub camera_buffer: wgpu::Buffer,   // binding 0: Camera (includes resolution)
    time_buffer: wgpu::Buffer,          // binding 1: time f32
    bind_group: wgpu::BindGroup,
    start_time: Instant,
    pub camera: Camera,
}

impl State {
    pub async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();

        let instance = wgpu::Instance::default();
        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                compatible_surface: Some(&surface),
                ..Default::default()
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_capabilities(&adapter).formats[0],
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        surface.configure(&device, &config);

        let camera = Camera::new(size.width, size.height);

        // Binding 0 — Camera (position + orientation + resolution)
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::bytes_of(&camera),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        // Binding 1 — time (f32, updated every frame)
        let time_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Time Buffer"),
            contents: bytemuck::cast_slice(&[0.0f32]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_entry = |binding: u32| wgpu::BindGroupLayoutEntry {
            binding,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        };

        let bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[uniform_entry(0), uniform_entry(1)],
            });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: time_buffer.as_entire_binding(),
                },
            ],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        let pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        Self {
            surface,
            device,
            queue,
            config,
            pipeline,
            camera_buffer,
            time_buffer,
            bind_group,
            start_time: Instant::now(),
            camera,
        }
    }

    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera.resolution = [new_size.width as f32, new_size.height as f32];
            self.flush_camera();
        }
    }

    /// Write current camera state to GPU buffer
    pub fn flush_camera(&self) {
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::bytes_of(&self.camera),
        );
    }

    pub fn render(&mut self) -> Result<(), SurfaceError> {
        // Update time and camera uniforms
        let elapsed = self.start_time.elapsed().as_secs_f32();
        self.queue.write_buffer(
            &self.time_buffer,
            0,
            bytemuck::cast_slice(&[elapsed]),
        );
        self.flush_camera();

        let output = self.surface.get_current_texture()?;
        let view = output.texture.create_view(&Default::default());

        let mut encoder = self.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor { label: None },
        );

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
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

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_bind_group(0, &self.bind_group, &[]);
            render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(Some(encoder.finish()));
        output.present();

        Ok(())
    }
}
