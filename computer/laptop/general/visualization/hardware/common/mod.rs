pub mod data_flow;
pub mod temperature;
pub mod colors;

pub use data_flow::DataFlow;
pub use temperature::TemperatureMap;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub position: Point,
    pub size: Size,
}

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
} 