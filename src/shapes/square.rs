use std::error::Error;

use super::Shape;
use super::Svg;

pub struct Square {
    width: f64,
    height: f64,
    color: Vec<u8>,
    border: Vec<f64>,
}

impl Square {
    pub fn new(width: f64, height: f64, color: Vec<u8>, border: Vec<f64>) -> Self {
        assert_eq!(color.len(), 4);
        Square {
            width,
            height,
            color: color.clone(),
            border: border.clone(),
        }
    }
}

impl Shape for Square {

    /// Sets the dimensions of the shape as a vec of length 2
    /// # Example
    /// ```rust
    /// # use crate::graph_lib::shapes::Square;
    /// # use crate::graph_lib::shapes::Shape;
    /// let c = Square::new(1.0, 1.0, vec![255, 0, 0, 255], vec![0.0, 0.0, 0.0, 0.0]); // The 1.0 & 1.0 are the width & height
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
    /// let mut c = Square::new(1.0, 1.0, vec![255, 0, 0, 255], vec![0.0, 0.0, 0.0, 0.0]);
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
    /// let c = Square::new(1.0, 1.0, vec![255, 0, 0, 255], vec![0.0, 0.0, 0.0, 0.0]);
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
    /// let mut c = Square::new(1.0, 1.0, vec![255, 0, 0, 255], vec![0.0, 0.0, 0.0, 0.0]);
    /// c.set_color(vec![0, 255, 0, 255]);
    /// assert_eq!(c.get_color(), vec![0, 255, 0, 255]);
    /// ```
    fn set_color(&mut self, color: Vec<u8>) -> Result<(), Box<dyn Error>> {
        assert_eq!(color.len(), 4);
        self.color = color;
        return Ok(());
    }

    /// Gets the border of the shape
    /// # Example
    /// ```rust
    /// # use crate::graph_lib::shapes::Square;
    /// # use crate::graph_lib::shapes::Shape;
    /// let c = Square::new(1.0, 1.0, vec![255, 0, 0, 255], vec![0.0, 0.0, 0.0, 0.0]);
    /// assert_eq!(c.get_border(), vec![0.0, 0.0, 0.0, 0.0]);
    /// ```
    fn get_border(&self) -> Vec<f64> {
        return self.border.clone();
    }

    /// Sets the border of the shape
    /// # Example
    /// ```rust
    /// # use crate::graph_lib::shapes::Square;
    /// # use crate::graph_lib::shapes::Shape;
    /// let mut c = Square::new(1.0, 1.0, vec![255, 0, 0, 255], vec![0.0, 0.0, 0.0, 0.0]);
    /// c.set_border(vec![5.0, 1.0, 1.0, 1.0]);
    /// assert_eq!(c.get_border(), vec![5.0, 1.0, 1.0, 1.0]);
    /// ```
    fn set_border(&mut self, border: Vec<f64>) -> Result<(), Box<dyn Error>> {
        assert_eq!(border.len(), 4);
        self.border = border.clone();
        return Ok(());
    }

    fn method(&self, x: f64, y: f64) -> bool {
        todo!()
    }
}

impl Svg for Square {
    fn get_tag(&self) -> String {
        todo!()
    }
}
