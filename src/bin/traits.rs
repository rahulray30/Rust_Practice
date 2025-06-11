use std::{cell::RefCell, fmt::format, ops, slice::Windows, u32};
fn main() {
    println!("inside traits");

  #[derive(Debug)]

  enum Months{
    January,
    February,
    March
  }

  
  impl Months{
    fn print_month(s: &str) -> Option<Self>{
        match s {
            "1" => Some(Months::January),
            "2" => Some(Months::February),
            "3" => Some(Months::March),

            _ => None
        }
    }

    fn ab (&self) ->  {

    }
  }

  let mon = Months::print_month("1");
  println!("mon {:?}", mon);






    // let value = RefCell::new(10);

    // {
    //     let mut borrow_mut = value.borrow_mut();
    //     *borrow_mut += 5;
    // }

    // println!("Value: {:?}", value.borrow());

 


    // let mut s1 = Sheep::new("abc".to_string());
    // // s1.is_naked();
    // println!("name of ship is {}", s1.name());
    // println!("{} ship is naked -  {}", s1.name(), s1.is_naked());
    // println!("noise of sheep abc is {}", s1.noise());
    // s1.shear();

    // println!("{} ship is naked -  {}", s1.name(), s1.is_naked());

    //Example 2

    // let _one_second = Seconds(1);

    // println!("One second looks like: {:?}", _one_second);
    // let _this_is_true = (_one_second == _one_second);
    // let _this_is_false = (_one_second > _one_second);

    // let foot = Inches(12);

    // println!("One foot equals {:?}", foot);

    // let meter = Centimeters(100.0);

    // let cmp =
    //     if foot.to_centimeters() < meter {
    //         "smaller"
    //     } else {
    //         "bigger"
    //     };

    // println!("One foot is {} than one meter.", cmp);

    //Example 3

    // assert_eq!(6, multiply(2u8, 3u8));
    // assert_eq!(5.0, multiply(1.0, 5.0));
    // assert_eq!(4, addition(3,1));

    // Example 4 ---------------------------------
    // DON'T modify the code below.
    // You need to derive some trait for FooBar to make it comparable.
    // assert_eq!(Foo + Bar, FooBar);
    // assert_eq!(Foo - Bar, BarFoo);

    // Example 5 ---------------------------------

    // let post = Post {
    //     title: "Popular Rust".to_string(),
    //     author: "Sunface".to_string(),
    //     content: "Rust is awesome!".to_string(),
    // };
    // let weibo = Weibo {
    //     username: "sunface".to_string(),
    //     content: "Weibo seems to be worse than Tweet".to_string(),
    // };

    // summary(&post);
    // summary(&weibo);

    // println!("{:?}", post);
    // println!("{:?}", weibo);

    // Example 6 ---------------------------------

    // let random_number = 0.234;
    // let animal = random_animal(random_number);
    // println!(
    //     "You've randomly chosen an animal, and it says {}",
    //     animal.noise()
    // );

    // assert_eq!(sum(1, 2), 3);

    // Example 7 ---------------------------------

    // let duck = Duck;
    // duck.swim();

    // let bird = hatch_a_bird(2);
    // assert_eq!(bird.quack(), "duckkkkk");

    // let bird = hatch_a_bird(1);
    // assert_eq!(bird.quack(), "swannnn");

    // Example 8 ---------------------------------

    // let birds: Vec<Box<dyn Bird>> = vec![Box::new(Duck), Box::new(Swan)];

    // for bird in birds {
    //     bird.quack();
    // }

    // Example 9 ---------------------------------

    //     let x = 1.1f64;
    //     let y = 8u8;

    //     let z = Box::new (&x);

    //    draw_with_box(Box::new (x));

    //    draw_with_ref(&y);

    // Example 10 ---------------------------------
    let x = 5u8;
    let y = "Hello".to_string();

    static_dispatch(&x);
    dynamic_dispatch(&y);
    println!("Success!");

    // Example 11 Closures -----------------------------------

    //  let mut lights_on = false;
    //  let mut temperature = 25;

    //  let mut lights_handler = EventHandler {
    //      on_event: || {
    //          lights_on = !lights_on;
    //          println!("Lights are now {}", if lights_on { "on" } else { "off" });
    //      },
    //  };

    //  let mut temperature_handler = EventHandler {
    //      on_event: || {
    //          temperature += 5;
    //          println!("Temperature increased to {}°C", temperature);
    //      },
    //  };

    //  lights_handler.handle_event();
    //  temperature_handler.handle_event();
    //  temperature_handler.handle_event();
    //  lights_handler.handle_event();

    //  assert_eq!(temperature, 35);
    //  assert_eq!(lights_on, true);


    // Example 12  Lifetimes  -----------------------------------

    let s1 = String::from("hello");
    let s2 = String::from("world!");

    let result = longest(&s1, &s2);
    println!("Longest: {}", result);


}


// struct Sheep {
//     naked: bool,
//     name: String,
// }

// trait Animal {
//     fn new(name: String) -> Self;
//     fn name(&self) -> String;
//     fn noise(&self) -> String;
//     fn talk(&self) {
//         println!("{} says {} ", self.name(), self.noise());
//     }
// }

// impl Sheep {
//     fn is_naked(&self) -> bool {
//         self.naked
//     }

//     fn shear(&mut self) {
//         if self.is_naked() {
//             println!("{} is already naked.", self.name);
//         } else {
//             println!("{} haircut is done", self.name);
//             self.naked = true;
//         }
//     }
// }

// impl Animal for Sheep {
//     fn new(name: String) -> Sheep {
//         Sheep { naked: false, name }
//     }

//     fn name(&self) -> String {
//         self.name.clone()
//     }

//     fn noise(&self) -> String {
//         "baahhhh".to_string()
//     }
// }

// Example 2

// #[derive(PartialEq, PartialOrd)]
// struct Centimeters(f64);

// `Inches`, a tuple struct that can be printed
// #[derive(Debug)]
// struct Inches(i32);

// impl Inches {
//     fn to_centimeters(&self) -> Centimeters {
//         let &Inches(inches) = self;

//         Centimeters(inches as f64 * 2.54)
//     }
// }

// ADD some attributes to make the code work!
// DON'T modify other code!
// #[derive(Debug,PartialEq, PartialOrd)]
// struct Seconds(i32);

// Example 3 ---------------------------------

// fn multiply<T: ops::Mul<Output = T>>(x: T, y: T) -> T {
//     x * y
// }

// fn addition<T: ops::Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }

// Example 4 ---------------------------------
// struct Foo;
// struct Bar;

// #[derive(PartialEq, Debug)]
// struct FooBar;

// #[derive(PartialEq, Debug)]
// struct BarFoo;

// The `std::ops::Add` trait is used to specify the functionality of `+`.
// Here, we make `Add<Bar>` - the trait for addition with a RHS of type `Bar`.
// The following block implements the operation: Foo + Bar = FooBar
// impl ops::Add<Bar> for Foo {
//     type Output = FooBar;

//     fn add(self, _rhs: Bar) -> FooBar {
//         FooBar
//     }
// }

// impl ops::Sub<Bar> for Foo {
//     type Output = BarFoo;

//     fn sub(self, _rhs: Bar) -> BarFoo {
//         BarFoo
//     }
// }

// Example 5 ---------------------------------

// trait Summary {
//     fn summarize(&self) -> String;
// }

// #[derive(Debug)]
// struct Post {
//     title: String,
//     author: String,
//     content: String,
// }

// impl Summary for Post {
//     fn summarize(&self) -> String {
//         format!("The author of post {} is {}", self.title, self.author)
//     }
// }

// #[derive(Debug)]
// struct Weibo {
//     username: String,
//     content: String,
// }

// impl Summary for Weibo {
//     fn summarize(&self) -> String {
//         format!("{} published a weibo {}", self.username, self.content)
//     }
// }

// fn summary(t: &impl Summary){
//     let _ = t.summarize();
// }

// Example 6 ---------------------------------
// struct Sheep {}
// struct Cow {}

// trait Animal {
//     fn noise(&self) -> String;
// }

// impl Animal for Sheep {
//     fn noise(&self) -> String {
//         "baaaaah!".to_string()
//     }
// }

// impl Animal for Cow {
//     fn noise(&self) -> String {
//         "moooooo!".to_string()
//     }
// }

// Returns some struct that implements Animal, but we don't know which one at compile time.
// FIX the errors here, you can make a fake random, or you can use trait object.
// fn random_animal(random_number: f64) -> Box<dyn Animal> {
//     if random_number < 0.5 {
//         Box::new(Sheep {})
//     } else {
//         Box::new(Cow {})
//     }
// }

// fn sum< T: ops::Add<Output = T>>(x: T, y: T) -> T {
//     x + y
// }

// Example 7 ---------------------------------

// trait Bird {
//     fn quack(&self) -> String;
// }
// struct Duck;
// impl Duck {
//     fn swim(&self) {
//         println!("Look, the duck is swimming")
//     }
// }
// struct Swan;
// impl Swan {
//     fn fly(&self) {
//         println!("Look, the duck.. oh sorry, the swan is flying")
//     }
// }

// impl Bird for Duck {
//     fn quack(&self) -> String {
//         "duckkkkk".to_string()
//     }
// }

// impl Bird for Swan {
//     fn quack(&self) -> String {
//         "swannnn".to_string()
//     }
// }

// fn hatch_a_bird(data_type: u32) -> Box<dyn Bird> {
//     if (data_type == 1) {
//         Box::new(Swan {})
//     } else {
//         Box::new(Duck {})
//     }
// }

// Example 8 ---------------------------------

trait Bird {
    fn quack(&self);
}

struct Duck;
impl Duck {
    fn fly(&self) {
        println!("Duck is flying");
    }
}

struct Swan;
impl Swan {
    fn fly(&self) {
        println!("swan is flying");
    }
}

impl Bird for Duck {
    fn quack(&self) {
        println!("{}", "duck");
    }
}

impl Bird for Swan {
    fn quack(&self) {
        println!("{}", "swan");
    }
}

// Example 9 ---------------------------------

trait Draw {
    fn draw(&self) -> String;
}

impl Draw for u8 {
    fn draw(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Draw for f64 {
    fn draw(&self) -> String {
        format!("f64: {}", *self)
    }
}

fn draw_with_box(x: Box<dyn Draw>) {
    x.draw();
}

fn draw_with_ref(x: &u8) {
    x.draw();
}

// Example 10 ---------------------------------
trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        "abc".to_string()
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

fn static_dispatch(x: &u8) {}

fn dynamic_dispatch(x: &dyn Foo) {}

struct Person<'a> {
    name: &'a str, // ERROR: lifetime of name not specified
}

// Example 11 Closures -----------------------------------

struct EventHandler<T>
where
    T: FnMut(),
{
    on_event: T,
}

impl<T> EventHandler<T>
where
    T: FnMut(),
{
    fn handle_event(&mut self) {
        (self.on_event)()
    }
}

// Example 12  Lifetimes  -----------------------------------

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

