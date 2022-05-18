pub struct Circle {
    name: &'static str,
}

pub struct Rectangle {
    name: &'static str,
}

pub trait Shape {
    fn get_name(&self) -> &'static str;

    fn draw(&self) {
        println!("Drawing : {}", self.get_name());
    }
}

impl Shape for Circle {
    fn get_name(&self) -> &'static str {
        self.name
    }
}

impl Shape for Rectangle {
    fn get_name(&self) -> &'static str {
        self.name
    }
}

pub enum ShapeType {
    CIRCLE,
    RECTANGLE,
}

pub struct ShapeFac;

impl ShapeFac {
    pub fn new_shape(shape_type: &ShapeType) -> Box<dyn Shape> {
        match shape_type {
            ShapeType::CIRCLE => Box::new(Circle { name: "Circle" }),
            ShapeType::RECTANGLE => Box::new(Rectangle { name: "Rectangle" }),
        }
    }
}