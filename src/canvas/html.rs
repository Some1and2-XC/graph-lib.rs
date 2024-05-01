use super::Canvas;
use super::Shape;

use std::error::Error;

pub struct Html {
    elements: Vec<Box<dyn Shape>>,
}

impl Html {
    pub fn new() {
        todo!()
    }
}


impl Canvas for Html {
    fn add_element(&mut self, element: Box<dyn Shape>) -> &Self {
        self.elements.push(element);
        return self;
    }
    fn get_elements(&self) -> &Vec<Box<dyn Shape>> {
        return &self.elements;
    }
    fn render_elements(&self) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
