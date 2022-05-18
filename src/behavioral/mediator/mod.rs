pub struct User {
   pub name: &'static str,
}

pub trait ChatRoom {
    fn show_message(&self, message: &'static str);
}

impl ChatRoom for User {
    fn show_message(&self, message: &'static str) {
        println!("{} - {}", self.name, message)
    }
}

impl User {
    pub fn set_name(&mut self, name: &'static str) {
        self.name = name
    }
    pub fn send_message(&self, message: &'static str) {
        self.show_message(message)
    }
}
