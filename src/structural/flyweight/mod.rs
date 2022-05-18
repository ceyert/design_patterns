trait ShapeFly {
    fn draw(&self);
}

pub struct CircleFly {
    pub color: &'static str,
    pub x: i32,
    pub y: i32,
    pub radius: i32,
}

impl CircleFly {
    pub fn new_instance(color: &'static str, x: i32, y: i32, radius: i32) -> Self {
        CircleFly {
            color,
            x,
            y,
            radius,
        }
    }

    pub fn get_color(&self) -> &'static str {
        self.color
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn get_radius(&self) -> i32 {
        self.radius
    }
}

impl ShapeFly for CircleFly {
    fn draw(&self) {
        println!(
            "Circle: Draw() [Color : {} , x : {} y : {} radius : {} ]",
            self.get_color(),
            self.get_x(),
            self.get_y(),
            self.get_radius()
        );
    }
}
