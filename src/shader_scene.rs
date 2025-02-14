use iced_widget::shader::{self, Viewport};
use iced_winit::core::{Element, Rectangle, Theme, mouse};
use iced_wgpu::wgpu;
use crate::shader_pipeline::Pipeline;

pub struct ShaderScene;

impl ShaderScene {
    pub fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct Primitive;

impl shader::Primitive for Primitive {
    fn prepare(
        &self,
        device: &iced_wgpu::wgpu::Device,
        queue: &iced_wgpu::wgpu::Queue,
        format: iced_wgpu::wgpu::TextureFormat,
        storage: &mut shader::Storage,
        _bounds: &Rectangle,
        viewport: &Viewport,
    ) {
        let window_size = viewport.physical_size();
        let window_size_tuple = (window_size.width, window_size.height);

        // debug: fixed viewport
        //let window_size_tuple = (200, 300);

        if !storage.has::<Pipeline>() {
            storage.store(Pipeline::new(device, format, window_size_tuple));
        }
    }

    fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        storage: &shader::Storage,
        target: &wgpu::TextureView,
        clip_bounds: &Rectangle<u32>,
    ) {
        let pipeline = storage.get::<Pipeline>().unwrap();
        pipeline.render(target, encoder, clip_bounds);
    }
}

impl<Message> shader::Program<Message> for ShaderScene {
    type State = ();
    type Primitive = Primitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive {
        Primitive
    }
}
