pub struct Memento {
    state: &'static str,
}

impl Memento {
    fn get_state(&self) -> &'static str {
        self.state
    }
}

pub struct Originator {
    pub state: &'static str,
}

impl Originator {
    pub fn set_state(&mut self, state: &'static str) {
        self.state = state
    }

    pub fn get_state(&self) -> &'static str {
        self.state
    }

    pub fn save_state_to_memento(&self) -> Memento {
        Memento {
            state: self.get_state(),
        }
    }

    pub fn get_state_from_memento(&mut self, memento: &mut Memento) {
        self.set_state(memento.get_state())
    }
}

pub struct CareTaker {
    pub memento_list: Vec<Memento>,
}

impl CareTaker {
    pub fn new_instance() -> Self {
        CareTaker {
            memento_list: Vec::new(),
        }
    }
    pub fn add(&mut self, state: Memento) {
        self.memento_list.push(state)
    }
    pub fn get(&mut self, index: usize) -> Option<&mut Memento> {
        self.memento_list.get_mut(index)
    }
}
