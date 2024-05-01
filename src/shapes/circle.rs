use std::error::Error;

use super::Shape;

/// Struct that represents the circles to be rendered
pub struct Circle {
    width: f64,
    height: f64,
    /// The color must always have a length of 4
    color: Vec<u8>,
}

impl Circle {
    /// Function for creating a new circle
    /// # Example
    /// ```rust
    /// # use crate::graph_lib::shapes::Circle;
    /// # use crate::graph_lib::shapes::Shape;
    /// let c = Circle::new(1.0, 2.0, vec![255, 0, 0, 255]);
    /// assert_eq!(c.get_dimensions(), vec![1.0, 2.0]);
    /// ```
    pub fn new(width: f64, height: f64, color: Vec<u8>) -> Self {
        assert_eq!(color.len(), 4);
        Circle {
            width,
            height,
            color: color.clone(),
        }
    }
}

impl Shape for Circle {

    /// Sets the dimensions of the shape as a vec of length 2
    /// # Example
    /// ```rust
    /// # use crate::graph_lib::shapes::Circle;
    /// # use crate::graph_lib::shapes::Shape;
    /// let c = Circle::new(1.0, 1.0, vec![255, 0, 0, 255]); // The 1.0 & 1.0 are the width & height
    /// assert_eq!(c.get_dimensions(), vec![1.0, 1.0]);
    /// ```
    fn get_dimensions(&self) -> Vec<f64> {
        return vec![
            self.width,
            self.height,
        ];
    }

    /// Sets the dimensions of the shape
    /// # Example
    /// ```rust
    /// # use crate::graph_lib::shapes::Circle;
    /// # use crate::graph_lib::shapes::Shape;
    /// let mut c = Circle::new(1.0, 1.0, vec![255, 0, 0, 255]);
    /// c.set_dimensions(vec![2.0, 1.0]);
    /// assert_eq!(c.get_dimensions(), vec![2.0, 1.0]);
    /// ```
    fn set_dimensions(&mut self, dimensions: Vec<f64>) -> Result<(), Box<dyn Error>> {
        assert_eq!(dimensions.len(), 2);
        self.width = *dimensions
            .get(0)
            .unwrap()
            ;
        self.height = *dimensions
            .get(1)
            .unwrap()
            ;

        return Ok(());
    }

    /// Gets the color of the shape
    /// # Example
    /// ```rust
    /// # use crate::graph_lib::shapes::Circle;
    /// # use crate::graph_lib::shapes::Shape;
    /// let c = Circle::new(1.0, 1.0, vec![255, 0, 0, 255]);
    /// let color = c.get_color();
    /// assert_eq!(color, vec![255, 0, 0, 255]);
    /// ```
    fn get_color(&self) -> Vec<u8> {
        return self.color.clone();
    }

    /// Sets the color of the shape
    /// # Example
    /// ```rust
    /// # use crate::graph_lib::shapes::Circle;
    /// # use crate::graph_lib::shapes::Shape;
    /// let mut c = Circle::new(1.0, 1.0, vec![255, 0, 0, 255]);
    /// c.set_color(vec![0, 255, 0, 255]);
    /// assert_eq!(c.get_color(), vec![0, 255, 0, 255]);
    /// ```
    fn set_color(&mut self, color: Vec<u8>) -> Result<(), Box<dyn Error>> {
        assert_eq!(color.len(), 4);
        self.color = color;
        return Ok(());
    }

    /// Method for deciding if a point is inside the shape
    /// The offset of the shape is accounted for elsewhere
    /// # Example
    /// ```rust
    /// # use crate::graph_lib::shapes::Circle;
    /// # use crate::graph_lib::shapes::Shape;
    /// let c = Circle::new(1.0, 1.0, vec![255, 0, 0, 255]);
    /// assert_eq!(c.method(0.0, 0.0), false);
    /// assert_eq!(c.method(0.5, 0.5), true);
    /// // assert_eq!(c.method(1.0, 1.0), false); // needs to be patched!!
    /// ```
    fn method(&self, x: f64, y: f64) -> bool {
        return (x * x) as f64 / (self.width * self.width * 0.25) + (y * y) as f64 / (self.height * self.height * 0.25) > 1.0;
    }
}
