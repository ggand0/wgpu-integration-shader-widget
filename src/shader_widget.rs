use iced_widget::{slider, button, text_input, center, shader, row, column, container, text};
use iced_winit::core::{Color, Element, Length, Length::*, Theme};
use iced_core::alignment::Horizontal;

use iced_wgpu::{wgpu, Renderer};
use iced_winit::runtime::Program;
use std::sync::Arc;
use image::GenericImageView;
use crate::shader_scene::ShaderScene;


pub struct TextureShader {
    pub scene: ShaderScene, // Custom scene object
    pub background_color: Color,
    pub input: String,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    BackgroundColorChanged(Color),
    InputChanged(String),
    Nothing,
}

impl TextureShader {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue) -> Self {
        let scene = ShaderScene::new();

     TextureShader { scene, background_color: Color::WHITE, input: String::default(), }
    }

    pub fn scene_mut(&mut self) -> Option<&mut ShaderScene> {
        Some(&mut self.scene)
    }
}

impl iced_winit::runtime::Program for TextureShader {
    type Theme = Theme;
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> iced_winit::runtime::Task<Message> {
        match message {
            Message::Nothing => {
                println!("Nothing");
            }
            Message::Tick => {
                //self.scene.update();
                println!("Tick");
            }
            Message::BackgroundColorChanged(color) => {
                self.background_color = color;
            }
            Message::InputChanged(input) => {
                self.input = input;
            }
        }
        iced_winit::runtime::Task::none()
    }

    fn view(&self) -> Element<Message, Theme, Renderer> {
        // Use the custom shader widget
        let shader_widget = shader(&self.scene)
            .width(Fill).height(Fill);
        
        let background_color = self.background_color;
        let other_ui = column![
            text("Slider Here Below").color(Color::WHITE),
            slider(0.0..=1.0, background_color.r, move |r| {
                Message::BackgroundColorChanged(Color {
                    r,
                    ..background_color
                })
            })
            .step(0.01),
            button("Nothing Button").on_press(Message::Nothing),
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .padding(20)
        .align_x(Horizontal::Center);

        center(
            container(
                column![
                    shader_widget,
                    other_ui,
                    text_input("Placeholder", &self.input)
                        .on_input(Message::InputChanged),
                ]
            ).align_x(Horizontal::Center)
        ).into()
    }
}
