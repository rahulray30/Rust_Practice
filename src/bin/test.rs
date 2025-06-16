fn main() {
    println!("inside test!");
    
    match divide(10.0, 20.0) {
        Ok(result) => println!("result is {}", result),
        Err(e) => println!("Error in {}", e),
        _ => println!("does not match any")
    }
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E)
// }

fn divide(x: f64, y: f64) -> Result<f64, String> {
    if(x == 0.0){
        Err("cannot divide by 0".to_string())
    } else {
        Ok(x/y)
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}


mod customer {
    use crate::front_of_house::hosting;
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}