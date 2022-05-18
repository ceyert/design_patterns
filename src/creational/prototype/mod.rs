pub trait IShape {
    fn draw(&self) {
        println!("N/A");
    }
    fn get_id(&self) -> &'static str;
    fn get_stype(&self) -> &'static str;
}


pub struct ShapeProto<T: IShape> {
   pub extend: T
}


impl<T: IShape + Clone> IShape for ShapeProto<T> {
    fn draw(&self) {
        println!("Inside {}", self.get_stype());
    }

    fn get_id(&self) -> &'static str {
        self.extend.get_id()
    }

    fn get_stype(&self) -> &'static str {
        self.extend.get_stype()
    }
}

#[derive(Clone)]
pub struct RectangleProto {
   pub id: &'static str,
   pub stype: &'static str,
}


impl IShape for RectangleProto {
    fn get_id(&self) -> &'static str {
        self.id
    }

    fn get_stype(&self) -> &'static str {
        self.stype
    }
}


impl ShapeProto<RectangleProto> {
   pub fn new_shape(id: &'static str) -> ShapeProto<RectangleProto> {
    ShapeProto { extend: RectangleProto { id, stype: "RectangleProto" } }
    }

    pub fn get_extend(&self) -> &RectangleProto {
        &self.extend
    }
}

#[derive(Clone)]
pub struct SquareProto {
    pub id: &'static str,
   pub stype: &'static str,
}

impl IShape for SquareProto {
    fn get_id(&self) -> &'static str {
        self.id
    }

    fn get_stype(&self) -> &'static str {
        self.stype
    }
}

impl ShapeProto<SquareProto> {
   pub fn new_shape(id: &'static str) -> ShapeProto<SquareProto> {
    ShapeProto { extend: SquareProto { id, stype: "SquareProto" } }
    }

    pub fn get_extend(&self) -> &SquareProto {
        &self.extend
    }
}

#[derive(Clone)]
pub struct CircleProto {
   pub id: &'static str,
    pub stype: &'static str,
}

impl IShape for CircleProto {
    fn get_id(&self) -> &'static str {
        self.id
    }

    fn get_stype(&self) -> &'static str {
        self.stype
    }
}

impl ShapeProto<CircleProto> {
   pub fn new_shape(id: &'static str) -> ShapeProto<CircleProto> {
    ShapeProto { extend: CircleProto { id, stype: "CircleProto" } }
    }

   pub fn get_extend(&self) -> &CircleProto {
        &self.extend
    }
}