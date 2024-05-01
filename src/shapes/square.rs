use std::error::Error;

use super::Shape;

pub struct Square {
    width: f64,
    height: f64,
    color: Vec<u8>,
}

impl Square {
    pub fn new(width: f64, height: f64, color: Vec<u8>) -> Self {
        assert_eq!(color.len(), 4);
        Square {
            width,
            height,
            color: color.clone(),
        }
    }
}

impl Shape for Square {

    /// Sets the dimensions of the shape as a vec of length 2
    /// # Example
    /// ```rust
    /// # use crate::graph_lib::shapes::Square;
    /// # use crate::graph_lib::shapes::Shape;
    /// let c = Square::new(1.0, 1.0, vec![255, 0, 0, 255]); // The 1.0 & 1.0 are the width & height
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
    /// # use crate::graph_lib::shapes::Square;
    /// # use crate::graph_lib::shapes::Shape;
    /// let mut c = Square::new(1.0, 1.0, vec![255, 0, 0, 255]);
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
    /// # use crate::graph_lib::shapes::Square;
    /// # use crate::graph_lib::shapes::Shape;
    /// let c = Square::new(1.0, 1.0, vec![255, 0, 0, 255]);
    /// let color = c.get_color();
    /// assert_eq!(color, vec![255, 0, 0, 255]);
    /// ```
    fn get_color(&self) -> Vec<u8> {
        return self.color.clone();
    }

    /// Sets the color of the shape
    /// # Example
    /// ```rust
    /// # use crate::graph_lib::shapes::Square;
    /// # use crate::graph_lib::shapes::Shape;
    /// let mut c = Square::new(1.0, 1.0, vec![255, 0, 0, 255]);
    /// c.set_color(vec![0, 255, 0, 255]);
    /// assert_eq!(c.get_color(), vec![0, 255, 0, 255]);
    /// ```
    fn set_color(&mut self, color: Vec<u8>) -> Result<(), Box<dyn Error>> {
        assert_eq!(color.len(), 4);
        self.color = color;
        return Ok(());
    }
}
