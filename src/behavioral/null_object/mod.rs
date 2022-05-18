pub trait AbsCustomer {
    fn get_name(&self) -> &'static str;
    fn is_nill(&self) -> bool;
}

pub struct RealCustomer {
    name: &'static str
}

impl RealCustomer {
   pub fn new_instance(name: &'static str) -> Self {
        RealCustomer { name }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }
}

impl AbsCustomer for RealCustomer {
    fn get_name(&self) -> &'static str {
        self.get_name()
    }

    fn is_nill(&self) -> bool {
        false
    }
}

pub struct NillCustomer;

impl NillCustomer {
    pub fn new_instance() -> Self {
        NillCustomer {}
    }

    pub fn get_name(&self) -> &'static str {
        "Not avaiable in Database"
    }
}

impl AbsCustomer for NillCustomer {
    fn get_name(&self) -> &'static str {
        self.get_name()
    }

    fn is_nill(&self) -> bool {
        true
    }
}


pub fn get_customer(name: &'static str) -> Box<dyn AbsCustomer> {
    let names: [&'static str; 3] = ["Rob", "Joe", "Julie"];
    if names.contains(&name) {
        Box::new(RealCustomer::new_instance(name))
    } else {
        Box::new(NillCustomer::new_instance())
    }
}