pub trait State {
    fn current_state(self: Box<Self>) -> Box<dyn State>;
    fn on_way(self: Box<Self>) -> Box<dyn State> {
        Box::new(())
    }

    fn delivered(self: Box<Self>) -> Box<dyn State> {
        Box::new(())
    }
    fn state_message<'a>(&self) -> &'static str {
        "Status : N/A"
    }
}

impl State for () {
    fn current_state(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

pub struct Empty;

impl State for Empty {
    fn current_state(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn state_message<'a>(&self) -> &'static str {
        "Status : Empty Basket"
    }
}

pub struct ReadyToBuy;

impl State for ReadyToBuy {
    fn current_state(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn state_message<'a>(&self) -> &'static str {
        "Status : Items ready to buy"
    }
}

pub struct Packaging;

impl State for Packaging {
    fn current_state(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn on_way(self: Box<Self>) -> Box<dyn State> {
        Box::new(OnWay)
    }

    fn state_message<'a>(&self) -> &'static str {
        "Status : On Packaging"
    }
}

pub struct OnWay;

impl State for OnWay {
    fn current_state(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn on_way(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn delivered(self: Box<Self>) -> Box<dyn State> {
        Box::new(Delivered)
    }

    fn state_message<'a>(&self) -> &'static str {
        "Status : On Way"
    }
}

pub struct Delivered;

impl State for Delivered {
    fn current_state(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn delivered(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn state_message<'a>(&self) -> &'static str {
        "Status : Delivered"
    }
}

pub struct Order {
    pub state: Option<Box<dyn State>>,
    pub shopping_status: ShoppingStatus,
}

impl Order {
    pub fn new_instance() -> Self {
        Order {
            state: Some(Box::new(Empty)),
            shopping_status: ShoppingStatus::Anony,
        }
    }

    pub fn state_message(&self) -> &'static str {
        self.state.as_ref().ok_or("NA").unwrap().state_message()
    }

    pub fn buy_orders(&mut self) {
        match self.shopping_status {
            ShoppingStatus::Payment => {
                if let Some(v) = self.state.take() {
                    self.state = Some(v.on_way());
                    println!("{}", self.state.as_ref().unwrap().state_message());
                    self.shopping_status = ShoppingStatus::Deliver;
                    self.buy_orders();
                }
            }
            ShoppingStatus::Deliver => {
                if let Some(v) = self.state.take() {
                    self.state = Some(v.delivered());
                }
            }
            ShoppingStatus::Anony => {
                println!("Error - {}", self.state.as_ref().unwrap().state_message());
            }
            ShoppingStatus::Login => {
                println!("Error - {}", self.state.as_ref().unwrap().state_message());
            }
            _ => {
                println!("General Error");
            }
        }
    }

    pub fn set_state(&mut self, shopping_status: ShoppingStatus) {
        match shopping_status {
            ShoppingStatus::Payment => {
                if let Some(_s) = self.state.take() {
                    self.state = Some(Box::new(Packaging));
                    self.shopping_status = ShoppingStatus::Payment
                }
            }
            ShoppingStatus::AddBasket => {
                if let Some(_s) = self.state.take() {
                    self.state = Some(Box::new(ReadyToBuy));
                    self.shopping_status = ShoppingStatus::AddBasket
                }
            }
            ShoppingStatus::Login => {
                if let Some(_s) = self.state.take() {
                    self.state = Some(Box::new(ReadyToBuy));
                    self.shopping_status = ShoppingStatus::Login
                }
            }
            ShoppingStatus::Anony => {
                if let Some(_s) = self.state.take() {
                    self.state = Some(Box::new(Empty));
                    self.shopping_status = ShoppingStatus::Anony
                }
            }
            _ => {}
        }
    }
}

pub enum ShoppingStatus {
    Anony,
    Login,
    Payment,
    Deliver,
    AddBasket,
}
