pub trait Strategy {
    fn do_operation(&self, num1: usize, num2: usize) -> usize;
}

pub struct OperationAdd;

pub struct OperationSubstract;

pub struct OperationMultiply;

impl Strategy for OperationAdd {
    fn do_operation(&self, num1: usize, num2: usize) -> usize {
        num1 + num2
    }
}

impl Strategy for OperationSubstract {
    fn do_operation(&self, num1: usize, num2: usize) -> usize {
        num1 - num2
    }
}

impl Strategy for OperationMultiply {
    fn do_operation(&self, num1: usize, num2: usize) -> usize {
        num1 * num2
    }
}

pub struct Context {
    strategy: Box<dyn Strategy>,
}

impl Context {
    pub fn new_instance(strategy: Box<dyn Strategy>) -> Context {
        Context { strategy }
    }

    pub fn set_strategy(&mut self, new_strategy: Box<dyn Strategy>) {
        self.strategy = new_strategy
    }

    pub fn execute_strategy(&self, num1: usize, num2: usize) -> usize {
        self.strategy.do_operation(num1, num2)
    }
}