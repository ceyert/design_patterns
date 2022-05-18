pub trait OrderCmd {
    fn execute(&mut self);
}

#[derive(Clone, Copy)]
pub struct Stock {
    pub name: &'static str,
    pub status: bool,
    pub total: u32,
}

impl Stock {
    pub fn new_instance(name: &'static str) -> Self {
        Stock {
            name,
            status: true,
            total: 0,
        }
    }

    pub fn buy(&self) {
        println!(
            "Buy Order => Stock Name : {}, Total : {}",
            self.get_name(),
            self.get_total()
        );
    }

    pub fn sell(&mut self) {
        if self.get_status() {
            println!(
                "Sell Order => Status : {} , Stock Name : {}, Total : {} ",
                self.get_status(),
                self.get_name(),
                self.get_total()
            );
        } else {
            println!(
                "Sell Order => Status : {} - Insufficient Order , Stock Name : {}, Total : {} ",
                self.get_status(),
                self.get_name(),
                self.get_total()
            );
        }
    }

    fn get_name(&self) -> &'static str {
        self.name
    }

    fn get_status(&self) -> bool {
        self.status
    }

    fn set_status(&mut self, new_status: bool) {
        self.status = new_status
    }

    fn get_total(&self) -> u32 {
        self.total
    }

    fn set_total(&mut self, new_total: u32) {
        self.total = new_total;
    }
}
pub struct BuyStock {
   pub abc_stock: Stock,
}

impl BuyStock {
    pub fn new_instance(abc_stock: Stock, buy_quantity: u32) -> (Self, Stock) {
        let mut new_stock = abc_stock;
        new_stock.set_total(abc_stock.total + buy_quantity);
        (
            BuyStock {
                abc_stock: new_stock,
            },
            new_stock,
        )
    }
}

impl OrderCmd for BuyStock {
    fn execute(&mut self) {
        self.abc_stock.buy();
    }
}

pub struct SellStock {
   pub abc_stock: Stock,
}

impl SellStock {
    pub fn new_instance(abc_stock: Stock, sell_quantity: u32) -> (Self, Stock) {
        let mut modify_stock = abc_stock;
        if abc_stock.get_total() >= sell_quantity {
            modify_stock.set_total(abc_stock.get_total() - sell_quantity);
            modify_stock.set_status(true);
            (
                SellStock {
                    abc_stock: modify_stock,
                },
                modify_stock,
            )
        } else {
            modify_stock.set_status(false);
            (
                SellStock {
                    abc_stock: modify_stock,
                },
                modify_stock,
            )
        }
    }
}

impl OrderCmd for SellStock {
    fn execute(&mut self) {
        self.abc_stock.sell()
    }
}
pub struct Broker<'a> {
   pub order_list: Vec<&'a mut dyn OrderCmd>,
}

impl<'a> Broker<'a> {
    pub fn new_instance() -> Self {
        Broker {
            order_list: Vec::new(),
        }
    }
    pub fn take_order(&mut self, order: &'a mut dyn OrderCmd) {
        self.order_list.push(order)
    }

    pub fn place_orders(&mut self) {
        for order in self.order_list.iter_mut() {
            Broker::process_order(*order)
        }
        self.order_list.clear()
    }

    pub fn process_order(order: &mut dyn OrderCmd) {
        order.execute()
    }
}
