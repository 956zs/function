use std::io;

fn main() {
    println!("this is the first function!");

    the_sec_function(); // call the second function
    println!("input a number:");
    let mut x = String::new();
        .read_line(&mut x).expect("failed to read line");
    the_third_function(x);
}

fn the_sec_function() {
    println!("this is the second function!");
}

