
pub trait Expression {
    fn interpret(&self, context: &'static str) -> bool;
}

pub struct TerminalExpression {
    data: &'static str
}

impl TerminalExpression {
    fn new(data: &'static str) -> Self {
        TerminalExpression { data }
    }
}

impl Expression for TerminalExpression {
    fn interpret(&self, context: &'static str) -> bool {
        if context.contains(self.data) {
            true
        } else {
            false
        }
    }
}

pub struct OrExpression<T: Expression, V: Expression> {
    expr1: T,
    expr2: V,
}

impl<T: Expression, V: Expression> OrExpression<T, V> {
    fn new(expr1: T, expr2: V) -> Self {
        OrExpression { expr1, expr2 }
    }
}

impl<T: Expression, V: Expression> Expression for OrExpression<T, V> {
    fn interpret(&self, context: &'static str) -> bool {
        self.expr1.interpret(context) || self.expr2.interpret(context)
    }
}


pub struct AndExpression<T: Expression, V: Expression> {
    expr1: T,
    expr2: V,
}

impl<T: Expression, V: Expression> AndExpression<T, V> {
    fn new(expr1: T, expr2: V) -> Self {
        AndExpression { expr1, expr2 }
    }
}

impl<T: Expression, V: Expression> Expression for AndExpression<T, V> {
    fn interpret(&self, context: &'static str) -> bool {
        self.expr1.interpret(context) || self.expr2.interpret(context)
    }
}


pub fn male_expression() -> OrExpression<TerminalExpression, TerminalExpression> {
    let robert = TerminalExpression::new("Robert");
    let john = TerminalExpression::new("John");
    OrExpression::new(robert, john)
}

pub fn married_woman_expression() -> AndExpression<TerminalExpression, TerminalExpression> {
    let robert = TerminalExpression::new("Julie");
    let john = TerminalExpression::new("Married");
    AndExpression::new(robert, john)
}






