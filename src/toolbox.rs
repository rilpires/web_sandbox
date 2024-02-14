use sdl2::pixels::Color;
use sdl2::event::Event;
use crate::datatype::*;


pub struct ToolBox {
    current_color_index: usize,
    available_colors: Vec<Color>,
    mouse_box: Vector2<usize>,
    points_per_paint : usize,
}

impl ToolBox {
    pub fn new() -> ToolBox {
        ToolBox {
            current_color_index: 0,
            available_colors: vec![
                Color::RGB(255, 255, 255),
                Color::RGB(0, 0, 0),
                Color::RGB(255, 0, 0),
                Color::RGB(0, 255, 0),
                Color::RGB(0, 0, 255),
            ],
            mouse_box: Vector2{x:15, y:15},
            points_per_paint: 55,
        }
    }

    pub fn mouse_box(&self) -> Vector2<usize> {
        self.mouse_box
    }

    pub fn points_per_paint(&self) -> usize {
        self.points_per_paint
    }

    pub fn current_color_index(&self) -> usize {
        self.current_color_index
    }

    pub fn available_colors(&self) -> &Vec<Color> {
        &self.available_colors
    }

    pub fn get_current_color(&self) -> &Color {
        &self.available_colors[self.current_color_index as usize]
    }

}