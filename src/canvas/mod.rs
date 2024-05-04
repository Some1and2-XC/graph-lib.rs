use super::shapes::Shape;
use std::error::Error;

pub trait Canvas {
    fn add_element(&mut self, element: CanvasItem) -> &Self;
    fn get_elements(&self) -> &Vec<CanvasItem>;
    fn render_elements(&self) -> Result<(), Box<dyn Error>>;
}

pub struct CanvasItem {
    offset_x: f64,
    offset_y: f64,
    shape: Box<dyn Shape>,
}

mod html;
pub use html::Html;
