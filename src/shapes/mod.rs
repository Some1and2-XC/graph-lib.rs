use std::error::Error;

/// Trait for shapes
/// Defines things such as the color, width, height and borders
pub trait Shape {
    fn get_dimensions(&self) -> Vec<f64>;
    fn set_dimensions(&mut self, dimensions: Vec<f64>) -> Result<(), Box<dyn Error>>;

    fn get_color(&self) -> Vec<u8>; 
    fn set_color(&mut self) -> Result<(), Box<dyn Error>>; 
}

mod circle;
pub use self::circle::Circle;
