fn main() {
    println!("inside ownership");

    // let x = String::from("Hello world");
    // let y = &x;
    // println!("{}, {}",x, y);

    // ---------------------1---------------------

    let s1 = String::from(("hiiiii"));
    let s2 = take_ownership(s1);

    println!("{}",s2);

    let p = give_ownership();
    println!("{}", p);

    let q = String::from("Hello World");

    print_str(&q);

    println!("{}", q);

    let w = String::from("Hello ");
    
    let mut e = w;

    e.push_str("World!");

    println!("Success!");


    // ----------------------2--------------------



    // ------------------------------------------



    // ------------------------------------------

    


}

// ------------Example 1 ------------------


fn take_ownership(s: String) -> String{
 println!("{:?}", s);
 s
}

fn give_ownership() -> String {
    let s = String::from("Hello");
    s
}

fn print_str(s: &String)  {
    println!("{}",s)
}



// ------------Example 2 ------------------


// ------------Example 3 ------------------
// ------------Example 4 ------------------
// ------------Example 5 ------------------
// ------------Example 6 ------------------
// ------------Example 7 ------------------
// ------------Example 8 ------------------
