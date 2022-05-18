
pub trait ShapeDec {
    fn draw(&self);
}

pub struct RectangleDec;

pub struct CircleDec;

impl ShapeDec for RectangleDec {
    fn draw(&self) {
        println!("Shape : Rectangle");
    }
}


impl ShapeDec for CircleDec {
    fn draw(&self) {
        println!("Shape : Circle");
    }
}

pub enum ShapeType { CIRCLE, RECTANGLE }

//*************************************

pub struct ShapeDecorator {
    decorated_shape: Box<dyn ShapeDec>
}

impl ShapeDecorator {
   pub fn new_instance(shape_type: &ShapeType) -> Box<Self> {
        match shape_type {
            ShapeType::CIRCLE => Box::new(ShapeDecorator { decorated_shape: Box::new(CircleDec) }),
            ShapeType::RECTANGLE => Box::new(ShapeDecorator { decorated_shape: Box::new(RectangleDec) }),
        }
    }
}

impl ShapeDec for ShapeDecorator {
    fn draw(&self) {
        self.decorated_shape.draw();
    }
}

pub trait RedShape {
    fn set_border_color(&self);
}


pub struct RedShapeDecorator {
    decorated_shape: Box<dyn ShapeDec>
}

impl RedShapeDecorator {
   pub fn new_instance(shape_type: &ShapeType) -> Box<Self> {
        match shape_type {
            ShapeType::CIRCLE => Box::new(RedShapeDecorator {
                decorated_shape: Box::new(CircleDec)
            }
            ),
            ShapeType::RECTANGLE => Box::new(RedShapeDecorator {
                decorated_shape: Box::new(RectangleDec)
            }
            ),
        }
    }
}


impl RedShape for RedShapeDecorator {
    fn set_border_color(&self) {
        println!("Border Color : RED")
    }
}



impl ShapeDec for RedShapeDecorator {
    fn draw(&self) {
        self.decorated_shape.draw();
        self.set_border_color()
    }
}

