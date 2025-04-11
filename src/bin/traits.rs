use std::ops;
fn main() {
    println!("inside traits");

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

example1();


    println!("Success!");
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
fn example1() {
    // `T: Trait` is the commonly used way.
    // `T: Fn(u32) -> u32` specifies that we can only pass a closure to `T`.
    struct Cacher<T: Fn(u32) -> u32> {
        calculation: T,
        value: Option<u32>,
    }

    impl<T: Fn(u32) -> u32> Cacher<T> {
        fn new(calculation: T) -> Cacher<T> {
            Cacher {
                calculation,
                value: None,
            }
        }

        fn value(&mut self, arg: u32) -> u32 {
            match self.value {
                Some(v) => v,
                None => {
                    let v = (self.calculation)(arg);
                    self.value = Some(v);
                    v
                },
            }
        }
    }

    let mut cacher = Cacher::new(|x| x+1);
    assert_eq!(cacher.value(10),12 );
    assert_eq!(cacher.value(15), 15);
}