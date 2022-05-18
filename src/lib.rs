pub mod behavioral;
pub mod creational;
pub mod structural;

#[cfg(test)]
mod tests {

    #[cfg(test)]
    mod behavioral_tests {

        use crate::behavioral::chain_of_responsibility::*;
        use crate::behavioral::command::*;
        use crate::behavioral::interpreter::*;
        use crate::behavioral::iter::*;
        use crate::behavioral::mediator::*;
        use crate::behavioral::memento::*;
        use crate::behavioral::null_object::*;
        use crate::behavioral::observer::*;
        use crate::behavioral::state::*;
        use crate::behavioral::strategy::*;
        use crate::behavioral::templete::*;

        #[test]
        pub fn chain_of_resp() {
            let mut job1 = Job1::new("Job1", "Zookeper");
            let job2 = Job1::new("Job2", "Guardian");
            job1.next_job(&job2);
            Engine::run(&job1);
        }

        #[test]
        pub fn command() {
            let mut broker = Broker::new_instance();

            let stock = Stock::new_instance("JP Morgan");

            let mut buy = BuyStock::new_instance(stock, 1100);
            broker.take_order(&mut buy.0);

            let mut sell1 = SellStock::new_instance(buy.1, 500);
            broker.take_order(&mut sell1.0);

            let mut sell2 = SellStock::new_instance(sell1.1, 500);
            broker.take_order(&mut sell2.0);

            let mut sell3 = SellStock::new_instance(sell2.1, 500);
            broker.take_order(&mut sell3.0);

            broker.place_orders();
        }

        #[test]
        fn interpreter() {
            let males = male_expression();
            let married_woman = married_woman_expression();
            println!("John is male? {}", males.interpret("John"));
            println!(
                "Julie is a married women? {}",
                married_woman.interpret("Married Julie")
            );
        }

        #[test]
        fn iterator() {
            let mut data = DataHolder::new_instance();
            data.add(10);
            data.add(20);
            data.add(30);
            let mut itertion = data.iteration();
            let has_next = itertion.has_next();
            println!("{}", has_next);
            let data = itertion.next();
            println!("{:?}", data);
            let data = itertion.next();
            println!("{:?}", data.ok_or(-1));
            let data = itertion.next();
            println!("{:?}", data.ok_or(-1));
            let has_next = itertion.has_next();
            println!("{}", has_next);
            let data = itertion.next();
            println!("{:?}", data.ok_or(-1));
        }

        #[test]
        fn mediator() {
            let user1 = User { name: "Julia" };
            let user2 = User { name: "John" };
            user1.send_message("Hi John");
            user2.send_message("Hi Julia");
        }

        #[test]
        fn memento() {
            let mut originator = Originator { state: "State 1" };
            let mut care_taker = CareTaker::new_instance();
            care_taker.add(originator.save_state_to_memento());
            originator.set_state("State 2");
            care_taker.add(originator.save_state_to_memento());
            originator.set_state("State 3");
            care_taker.add(originator.save_state_to_memento());
            originator.set_state("State 4");
            println!("Current State : {}", originator.get_state());
            originator.get_state_from_memento(care_taker.get(0).unwrap());
            println!("First saved state : {}", originator.get_state());
            originator.get_state_from_memento(care_taker.get(1).unwrap());
            println!("Second saved state : {}", originator.get_state())
        }

        #[test]
        fn null_object() {
            let customer1 = get_customer("Rob");
            let customer2 = get_customer("Bob");
            let customer3 = get_customer("Julie");
            let customer4 = get_customer("Laura");

            println!("{}", customer1.get_name());
            println!("{}", customer2.get_name());
            println!("{}", customer3.get_name());
            println!("{}", customer4.get_name());
        }

        #[test]
        fn observer() {
            let mut observables = Observables::new_instance();

            let observer_data1 = ObserverData { data: "Hi 1" };
            let observer_data2 = ObserverData { data: "Hi 2" };
            let observer_data3 = ObserverData { data: "Hi 3" };
            observables.attach(&observer_data1);
            observables.attach(&observer_data2);
            observables.detach(&observer_data1);
            observables.detach(&observer_data2);
            observables.detach(&observer_data3);
        }

        #[test]
        fn state() {
            let mut shop = Order::new_instance();

            let mut status_message = shop.state_message();
            println!("{}", status_message);
            shop.set_state(ShoppingStatus::Login);
            status_message = shop.state_message();
            println!("{}", status_message);
            shop.set_state(ShoppingStatus::AddBasket);
            status_message = shop.state_message();
            println!("{}", status_message);
            shop.set_state(ShoppingStatus::Payment);
            status_message = shop.state_message();
            println!("{}", status_message);
            shop.buy_orders();
            status_message = shop.state_message();
            println!("{}", status_message);
        }

        #[test]
        fn strategy() {
            let mut context: Context = Context::new_instance(Box::new(OperationAdd));

            let result = context.execute_strategy(5, 5);
            println!("Result Add : {}", result);
            context.set_strategy(Box::new(OperationMultiply));
            let result = context.execute_strategy(5, 5);
            println!("Result Multiply : {}", result);
        }

        #[test]
        fn templete() {
            let cricket = Cricket;
            cricket.play();
            let football = Football;
            football.play();
        }
    }

    #[cfg(test)]
    mod creational_tests {
        use crate::creational::abs_factory::*;
        use crate::creational::builder::*;
        use crate::creational::factory::*;
        use crate::creational::prototype::*;
        use crate::creational::singleton::*;
        use std::collections::HashMap;

        #[test]
        fn factory() {
            let shape = ShapeFac::new_shape(&ShapeType::CIRCLE);
            shape.draw();
            let shape2: Box<dyn Shape> = ShapeFac::new_shape(&ShapeType::RECTANGLE);
            shape2.draw();
        }

        #[test]
        fn abs_factory() {
            let draw = Draw::get_draw(&DrawType::SHAPE);
            let shape_option = draw.create_shape(&ShapeTypeAbs::RECTANGLE);
            match shape_option {
                None => println!("Wrong Type"),
                Some(shape) => {
                    shape.draw();
                }
            }
            let draw = Draw::get_draw(&DrawType::FORM);
            let form_option = draw.create_form(&FormType::CUBE);
            match form_option {
                None => println!("Wrong Type"),
                Some(form) => {
                    form.draw();
                }
            }
            let draw = Draw::get_draw(&DrawType::FORM);
            let shape_option = draw.create_shape(&ShapeTypeAbs::CIRCLE);
            match shape_option {
                None => println!("Wrong Type"),
                Some(shape) => {
                    shape.draw();
                }
            }
        }

        #[test]
        fn builder() {
            let veg_menu = MealBuilder::prepare_veg_meal(&DrinkType::COKE);
            for item in veg_menu.items.iter() {
                println!(
                    "Item Name : {} , Price : {}",
                    item.get_name(),
                    item.get_price()
                );
            }
            let non_veg_menu = MealBuilder::prepare_non_veg_meal(&DrinkType::PEPSI);
            for item in non_veg_menu.items.iter() {
                println!(
                    "Item Name : {} , Price : {}",
                    item.get_name(),
                    item.get_price()
                );
            }
        }

        #[test]
        fn prototype() {
            let mut shapes: HashMap<&str, Box<dyn IShape>> = HashMap::new();

            {
                let rectangle = ShapeProto::<RectangleProto>::new_shape("1");
                let circle = ShapeProto::<CircleProto>::new_shape("2");
                let square = ShapeProto::<SquareProto>::new_shape("3");
                shapes.insert(rectangle.get_extend().get_id(), Box::new(rectangle));
                shapes.insert(circle.get_extend().get_id(), Box::new(circle));
                shapes.insert(square.get_extend().get_id(), Box::new(square));
            }
            let rectangle_clone = shapes.get("1").unwrap().clone();
            rectangle_clone.draw();
            let circle_clone = shapes.get("2").unwrap().clone();
            circle_clone.draw();
            let square_clone = shapes.get("3").unwrap().clone();
            square_clone.draw();
            let rectangle_clone2 = shapes.get("1").unwrap().clone();
            rectangle_clone2.draw();
        }

        #[test]
        fn singleton() {
            let f1 = db_connection();
            println!("{:?}", f1);
            let f2 = db_connection();
            println!("{:?}", f2);
        }
    }

    #[cfg(test)]
    mod structural_tests {
        use std::collections::HashMap;
        use crate::structural::bridge::*;
        use crate::structural::composite::*;
        use crate::structural::decorator::*;
        use crate::structural::facade::*;
        use crate::structural::filter::*;
        use crate::structural::flyweight::*;

        #[test]
        fn bridge() {
            let red_circle = &RedCircle;
            let red_shape = Shape::new_circle(10, 10, 20, red_circle);
            red_shape.get_shape().draw();

            let green_circle = &GreenCircle;
            let green_shape = Shape::new_circle(10, 10, 20, green_circle);
            green_shape.get_shape().draw();
        }

        #[test]
        fn composite() {
            let mut ceo = Employee::new("John", "CEO", 50000);
            let mut head_sales = Employee::new("Robert", "Sales", 20000);
            let mut head_marketing = Employee::new("Michael", "Marketing", 15000);
            let mut senior_engineer1 = Employee::new("Andry", "Engineer", 5000);
            let mut senior_engineer2 = Employee::new("Joseph", "Engineer", 5000);
            let junior_engineer1 = Employee::new("Emily", "Junior Engineer", 3000);
            let junior_engineer2 = Employee::new("Clark", "Junior Engineer", 3000);
            senior_engineer1.add(&junior_engineer1);
            senior_engineer1.add(&junior_engineer2);
            senior_engineer2.add(&junior_engineer1);
            senior_engineer2.add(&junior_engineer2);
            head_marketing.add(&senior_engineer1);
            head_marketing.add(&senior_engineer2);
            head_sales.add(&head_marketing);
            ceo.add(&head_sales);
            println!("{}", ceo);
            ceo.remove(&head_sales);
            println!("{}", ceo);
            head_sales.remove(&head_marketing);
            head_marketing.remove(&senior_engineer1);
            head_marketing.remove(&senior_engineer2);
            senior_engineer1.remove(&junior_engineer1);
            senior_engineer1.remove(&junior_engineer2);
            senior_engineer2.remove(&junior_engineer1);
            senior_engineer2.remove(&junior_engineer2);
            println!("{}", senior_engineer1);
            println!("{}", senior_engineer2);
            println!("{}", junior_engineer1);
            println!("{}", junior_engineer2);
        }

        #[test]
        fn decorator() {
            let shape_decorator = ShapeDecorator::new_instance(&ShapeType::CIRCLE);
            shape_decorator.draw();
            let red_shape_decorator = RedShapeDecorator::new_instance(&ShapeType::RECTANGLE);
            red_shape_decorator.draw();
        }
        #[test]
        fn face() {
            let circle = CircleFac::new();
            let rectangle = RectangleFac::new();
            let square = SquareFac::new();
            let shape_maker = ShapeMaker::new(&circle, &rectangle, &square);
            shape_maker.draw_circle();
            shape_maker.draw_rectangle();
            shape_maker.draw_square();
        }

        #[test]
        fn filter() {
            let person = Person::new("Robert", "Male", "Single");
            let person2 = Person::new("John", "Male", "Married");
            let person3 = Person::new("Laura", "Female", "Married");
            let person4 = Person::new("Diana", "Female", "Single");
            let mut persons: Vec<Person> = Vec::new();
            persons.push(person);
            persons.push(person2);
            persons.push(person3);
            persons.push(person4);
            let male = CritariaMale;
            let male_criteria = male.meet_criteria(&persons);
            for person in male_criteria.into_iter() {
                println!("Male : {}", person.get_name());
            }
            let female = CriteriaFemale;
            let female_criteria = female.meet_criteria(&persons);
            for person in female_criteria.into_iter() {
                println!("Female : {}", person.get_name());
            }
            let single = CriteriaSingle;
            let single_criteria = single.meet_criteria(&persons);
            for person in single_criteria.into_iter() {
                println!("Singles : {}", person.get_name());
            }
            let single_male = AndCriteria::new(&CriteriaSingle, &CritariaMale);
            let single_male_criteria = single_male.meet_criteria(&persons);
            for person in single_male_criteria.into_iter() {
                println!("Single And Male : {}", person.get_name());
            }
            let single_female = AndCriteria::new(&CriteriaSingle, &CriteriaFemale);
            let single_female_criteria = single_female.meet_criteria(&persons);
            for person in single_female_criteria.into_iter() {
                println!("Single And Female : {}", person.get_name());
            }
            let single_or_famele = OrCriteria::new(&CriteriaSingle, &CriteriaFemale);
            let single_or_famele_criteria = single_or_famele.meet_criteria(&persons);
            for person in single_or_famele_criteria.into_iter() {
                println!("Single Or Famele : {}", person.get_name());
            }
            let single_or_male = OrCriteria::new(&CriteriaSingle, &CritariaMale);
            let single_or_male_criteria = single_or_male.meet_criteria(&persons);
            for person in single_or_male_criteria.into_iter() {
                println!("Single Or Male : {}", person.get_name());
            }
        }

        #[test]
        fn flyweight() {
            let mut circles: HashMap<&'static str, CircleFly> = HashMap::new();
            let circle1 = CircleFly::new_instance("Red", 10, 10, 5);
            let circle2 = CircleFly::new_instance("Blue", 10, 10, 5);
            let circle3 = CircleFly::new_instance("Green", 10, 10, 5);
            let circle4 = CircleFly::new_instance("Red", 10, 10, 5);
            let circle5 = CircleFly::new_instance("Blue", 10, 10, 5);
        
            circles.insert(circle1.color, circle1);
            circles.insert(circle2.color, circle2);
            circles.insert(circle3.color, circle3);
            circles.insert(circle4.color, circle4);
            circles.insert(circle5.color, circle5);
        
        
            for circle in circles {
                println!("{}", circle.0)
            }
        
        }
    }
}
