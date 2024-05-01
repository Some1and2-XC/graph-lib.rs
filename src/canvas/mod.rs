use super::shapes::Shape;
use std::error::Error;

pub trait Canvas {
    fn add_element(&mut self, element: Box<dyn Shape>) -> &Self;
    fn get_elements(&self) -> &Vec<Box<dyn Shape>>;
    fn render_elements(&self) -> Result<(), Box<dyn Error>>;
}

mod html;
pub use html::Html;
