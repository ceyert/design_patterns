pub trait Iterator<E> {
    fn next(&mut self) -> Option<E>;
    fn has_next(&self) -> bool;
}

pub struct DataHolder<E> {
    data: Vec<E>,
}

impl<E> DataHolder<E> {
    pub fn new_instance() -> Self {
        DataHolder { data: Vec::new() }
    }

    pub fn add(&mut self, item: E) {
        self.data.push(item)
    }

    pub fn iteration(&self) -> Container<E> {
        Container::new_instance(self)
    }
}

pub struct Container<'a, E> {
    index: usize,
    data_holder: &'a DataHolder<E>,
}

impl<'a, E> Container<'a, E> {
    pub fn new_instance(data_holder: &'a DataHolder<E>) -> Container<E> {
        Container {
            index: 0,
            data_holder,
        }
    }
}

impl<'a, E: Clone> Iterator<E> for Container<'a, E> {
    fn next(&mut self) -> Option<E> {
        let current = self.data_holder.data.get(self.index).cloned();
        self.index += 1;
        current
    }

    fn has_next(&self) -> bool {
        self.data_holder.data.len() > self.index
    }
}
