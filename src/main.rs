// use std::io;

use std::collections::HashMap;

fn main() {
    println!("Guess the numbervvvvv!");

    let mut manager = StudentManager::new();

    let student1 = Student {
        id: 1,
        name: String::from("Alice"),
        grade: String::from("A"),
    };
    let student2 = Student {
        id: 2,
        name: String::from("Bob"),
        grade: String::from("B"),
    };

    manager.add_student(student1).unwrap();
    manager.add_student(student2).unwrap();

    let _user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // println!("user 1 details {:#?}",user1);

    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };
    // println!("area of the reactanlge is {}", rect1.area());
    let mut s = String::from("hellooooooo world");
    let sbytes = s.as_bytes();

    let word = first_word(&s);

    // println!("slice expample, {word}");
    // println!("bytes data {:?}", sbytes);
    // println!("bytes data {:x?}", sbytes);

    //enum

    let some_number = Some(5);
    let some_char = Some('e');

    let mut absent_number: Option<i32> = None;
    absent_number = Some(3);

    let value = value_in_cents(Coin::Nickel);

    println!("inside value penny {} ", value);

    // println!("absent_number , {:?}", absent_number);

    // println!("some number {:?}", some_number);

    // GENERICS

    let p1 = Point::new(1.0, 2.0);
    p1.printing();
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    // println!("bytes of data, {bytes}");

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}



fn dangle() -> String {
    let s = String::from("hello");
    println!("s--- {}", s);
    s
}

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//Struct

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
}

//ENUM

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enum Option<T> {
//     None,
//     Some(T),
// }

// struct Car{
//     owner: String,
//     year: u32,
//     fuel_level: f32
// }

// practice

struct Student {
    id: i32,
    name: String,
    grade: String,
}

struct StudentManager {
    students: HashMap<i32, Student>,
}

impl StudentManager {
    fn new() -> Self {
        StudentManager {
            students: HashMap::new(),
        }
    }

    fn add_student(&mut self, student: Student) -> Result<(), String> {
        if self.students.contains_key(&student.id) {
            Err(format!("student with ID {} already exists ", student.id))
        } else {
            self.students.insert(student.id, student);
            Ok(())
        }
    }

    fn get_student(&self, id: i32) -> Option<&Student> {
        self.students.get(&id)
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Point<T, U> {
        Point { x, y }
    }
}

impl Point<f32, f32> {
    fn printing(&self) {
        println!("x: {}, y: {}", self.x, self.y);
    }
}

fn add_points<T, U>(p1: Point<T, U>, p2: Point<T, U>) -> Point<T, U> {
    unimplemented!();
}



