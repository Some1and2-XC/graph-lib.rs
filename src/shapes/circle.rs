use std::error::Error;

use super::Shape;

pub struct Circle {
    width: f64,
    height: f64,

    color: Vec<u8>,
}

impl Circle {
    /// Function for creating a new circle
    /// # Example
    /// ```rust
    /// # use lib::shapes::Circle;
    /// # use lib::shapes::Shape;
    /// let c = Circle::new(1.0, 2.0, vec![1, 112, 245]);
    /// assert_eq!(c.get_dimensions(), vec![1.0, 2.0]);
    /// ```
    pub fn new(width: f64, height: f64, color: Vec<u8>) -> Self {
        let mut new_color = color.clone();
        new_color.resize(4, 0);
        Circle {
            width,
            height,
            color: new_color,
        }
    }
}

impl Shape for Circle {

    /// Sets the dimensions of the shape as a vec of length 2
    /// # Example
    /// ```rust
    /// # use lib::shapes::Circle;
    /// # use lib::shapes::Shape;
    /// let c = Circle::new(1.0, 1.0, vec![1, 2]); // The 1.0 & 1.0 are the width & height
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
    /// # use lib::shapes::Circle;
    /// # use lib::shapes::Shape;
    /// let mut c = Circle::new(1.0, 1.0, vec![1, 2]);
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

    fn get_color(&self) -> Vec<u8> { todo!() }
    fn set_color(&mut self) -> Result<(), Box<dyn Error>> { todo!() }

    fn get_offset(&self) -> Vec<f64> { todo!() }
    fn set_offset(&mut self, offset: Vec<f64>) -> Result<(), Box<dyn Error>> { todo!() }
}
