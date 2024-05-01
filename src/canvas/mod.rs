use super::shapes::Shape;

pub trait Canvas {
    fn add_element() -> Self;
    fn get_elements() -> Vec<Box<dyn Shape>>;
}

mod html;
pub use html::Html;
