use iced_wgpu::wgpu;
use iced_wgpu::wgpu::util::DeviceExt;
use iced_core::Rectangle;

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl Pipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, _window_size: (u32, u32)) -> Self {
        let vertices: [f32; 6] = [
            -0.5, -0.5, // Bottom left
             0.5, -0.5, // Bottom right
             0.0,  0.5, // Top center
        ];

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let indices: &[u16] = &[0, 1, 2];
        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Triangle Index Buffer"),
            contents: bytemuck::cast_slice(indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Triangle Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./triangle.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Triangle Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: 2 * std::mem::size_of::<f32>() as u64, 
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[wgpu::VertexAttribute {
                        offset: 0,
                        shader_location: 0,
                        format: wgpu::VertexFormat::Float32x2,
                    }],
                }],
            },
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            pipeline,
            vertices: vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        }
    }

    pub fn render(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        clip_bounds: &Rectangle<u32>,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            occlusion_query_set: None,
            timestamp_writes: None,
        });

        pass.set_pipeline(&self.pipeline);
        pass.set_vertex_buffer(0, self.vertices.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
