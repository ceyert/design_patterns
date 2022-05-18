
pub trait DrawApi {
    fn draw_circle(&self, radius: u32, x: i32, y: i32);
}

pub struct RedCircle;

pub struct GreenCircle;

impl DrawApi for RedCircle {
    fn draw_circle(&self, radius: u32, x: i32, y: i32) {
        println!("Drawing Circle[ color:red, radius:{}, x:{}, y:{} ]", radius, x, y);
    }
}

impl DrawApi for GreenCircle {
    fn draw_circle(&self, radius: u32, x: i32, y: i32) {
        println!("Drawing Circle[ color:green, radius:{}, x:{}, y:{} ]", radius, x, y);
    }
}
pub trait IShape {
    fn draw(&self);
}

pub struct Shape<T: IShape> {
    pub shape: T,
}

impl<T: IShape> Shape<T> {
    pub fn new(shape: T) -> Shape<T> {
        Shape { shape }
    }

   pub fn get_shape(self) -> T {
        self.shape
    }
}

pub struct Circle<'a, U: DrawApi> {
    x: i32,
    y: i32,
    radius: u32,
    draw_api: &'a U,
}

impl<'a, U: DrawApi> Circle<'a, U> {
   pub fn new_circle(x: i32, y: i32, radius: u32, draw_api: &'a U) -> Circle<U> {
    Circle { x, y, radius, draw_api }
    }
}

impl<'a, U: DrawApi> IShape for Circle<'a, U> {
    fn draw(&self) {
        self.draw_api.draw_circle(self.radius, self.x, self.y)
    }
}

impl<'a, U: DrawApi> Shape<Circle<'a, U>> {
   pub fn new_circle(x: i32, y: i32, radius: u32, draw_api: &'a U) -> Shape<Circle<'a, U>> {
        Shape::new(Circle::new_circle(x, y, radius, draw_api))
    }
}


