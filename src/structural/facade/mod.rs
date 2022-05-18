pub trait ShapeFac {
    fn draw(&self);
}

pub struct RectangleFac {}

impl RectangleFac {
    pub fn new() -> Self {
        RectangleFac {}
    }
}

impl ShapeFac for RectangleFac {
    fn draw(&self) {
        println!("Rectangle::draw()");
    }
}


pub struct SquareFac {}

impl SquareFac {
   pub fn new() -> Self {
        SquareFac {}
    }
}


impl ShapeFac for SquareFac {
    fn draw(&self) {
        println!("Square::draw()");
    }
}


pub struct CircleFac {}

impl CircleFac {
   pub fn new() -> Self {
        CircleFac {}
    }
}

impl ShapeFac for CircleFac {
    fn draw(&self) {
        println!("Circle::draw()");
    }
}

pub struct ShapeMaker<'a, T: ShapeFac, V: ShapeFac, Y: ShapeFac> {
    circle: &'a T,
    rectangle: &'a V,
    square: &'a Y,
}

impl<'a, T: ShapeFac, V: ShapeFac, Y: ShapeFac> ShapeMaker<'a, T, V, Y> {
   pub fn new(circle: &'a T, rectangle: &'a V, square: &'a Y) -> Self {
        ShapeMaker { circle, rectangle, square }
    }

   pub fn draw_circle(&self) {
        self.circle.draw();
    }

   pub fn draw_rectangle(&self) {
        self.rectangle.draw();
    }

   pub fn draw_square(&self) {
        self.square.draw();
    }
}