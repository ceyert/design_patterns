use std::fmt;

#[derive(std::cmp::PartialEq, Debug)]
pub struct Employee<'a> {
    name: &'static str,
    dept: &'static str,
    salary: u32,
    subordinates: Vec<&'a Employee<'a>>,
}

impl<'a> Employee<'a> {
   pub fn new(name: &'static str, dept: &'static str, salary: u32) -> Self {
        Employee { name, dept, salary, subordinates: Vec::new() }
    }

    pub fn add(&mut self, e: &'a Employee<'a>) {
        self.subordinates.push(e)
    }

    pub fn remove(&mut self, e: &'a Employee<'a>) {
        self.subordinates.remove(self.subordinates.iter().position(|emp| emp.eq(&e)).unwrap());
    }
}

impl<'a> fmt::Display for Employee<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {:?})", self.name, self.dept, self.salary, self.subordinates)
    }
}