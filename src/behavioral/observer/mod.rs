 use std::fmt::Debug;

pub trait IObserver {
    fn update_attach(&self);
    fn update_detach(&self);
}


pub trait IObservable<'a, T: IObserver> {
    fn attach(&mut self, observer: &'a T);
    fn detach(&mut self, observer: &'a T);
}


pub struct Observables<'a, T: IObserver> {
    data: Vec<&'a T>
}


impl<'a, T: IObserver> Observables<'a, T> {
   pub fn new_instance() -> Observables<'a, T> {
        Observables { data: Vec::new() }
    }
}

impl<'a, T: IObserver + PartialEq + Debug> IObservable<'a, T> for Observables<'a, T> {
    fn attach(&mut self, observer: &'a T) {
        self.data.push(observer);
        observer.update_attach();
    }

    fn detach(&mut self, observer: &'a T) {
        if let Some(id) = self.data.iter().position(|observer_in_list| *observer_in_list == observer) {
            self.data.remove(id);
            observer.update_detach();
        } else {
            println!("Data not attached : {:?}", observer)
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ObserverData<T> {
    pub data: T
}

impl<T: Debug> IObserver for ObserverData<T> {
    fn update_attach(&self) {
        println!("Data attached : {:?}", self.data)
    }

    fn update_detach(&self) {
        println!("Data detached: {:?}", self.data)
    }
}