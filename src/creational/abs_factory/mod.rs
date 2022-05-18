pub struct CircleAbs {
    name: &'static str,
}

pub struct RectangleAbs {
    name: &'static str,
}

pub trait ShapeAbs {
    fn get_name(&self) -> &'static str;

    fn draw(&self) {
        println!("Drawing Shape : {}", self.get_name());
    }
}

impl ShapeAbs for CircleAbs {
    fn get_name(&self) -> &'static str {
        self.name
    }
}

impl ShapeAbs for RectangleAbs {
    fn get_name(&self) -> &'static str {
        self.name
    }
}
pub struct CubeAbs {
    name: &'static str,
}

pub struct SphereAbs {
    name: &'static str,
}

pub trait Form {
    fn get_name(&self) -> &'static str;

    fn draw(&self) {
        println!("Drawing Form : {}", self.get_name());
    }
}

impl Form for CubeAbs {
    fn get_name(&self) -> &'static str {
        self.name
    }
}

impl Form for SphereAbs {
    fn get_name(&self) -> &'static str {
        self.name
    }
}

pub enum ShapeTypeAbs {
    CIRCLE,
    RECTANGLE,
}

pub enum FormType {
    CUBE,
    SPHERE,
}

pub trait ABSFactory {
    fn create_shape(&self, shape_type: &ShapeTypeAbs) -> Option<Box<dyn ShapeAbs>>;
    fn create_form(&self, form_type: &FormType) -> Option<Box<dyn Form>>;
}

pub struct ShapeFactory;

pub struct FormFactory;

impl ABSFactory for ShapeFactory {
    fn create_shape(&self, shape_type: &ShapeTypeAbs) -> Option<Box<dyn ShapeAbs>> {
        match shape_type {
            ShapeTypeAbs::CIRCLE => Some(Box::new(CircleAbs { name: "Circle" })),
            ShapeTypeAbs::RECTANGLE => Some(Box::new(RectangleAbs { name: "Rectangle" }))
        }
    }

    fn create_form(&self, _form_type: &FormType) -> Option<Box<dyn Form>> {
        None
    }
}

impl ABSFactory for FormFactory {
    fn create_shape(&self, _shape_type: &ShapeTypeAbs) -> Option<Box<dyn ShapeAbs>> {
        None
    }

    fn create_form(&self, form_type: &FormType) -> Option<Box<dyn Form>> {
        match form_type {
            FormType::CUBE => Some(Box::new(CubeAbs { name: "Cube" })),
            FormType::SPHERE => Some(Box::new(SphereAbs { name: "Sphere" }))
        }
    }
}
pub enum DrawType {
    SHAPE,
    FORM,
}

pub struct Draw;

impl Draw {
   pub fn get_draw(draw_type: &DrawType) -> Box<dyn ABSFactory> {
        match draw_type {
            DrawType::FORM => Box::new(FormFactory {}),
            DrawType::SHAPE => Box::new(ShapeFactory {}),
        }
    }
}