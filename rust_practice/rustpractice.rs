use std::io;

// weird
use std::io::prelude::*;

// you MUST specify the var type, but the thing you pass in
// doesn't need to have been specified as this type
fn something(x: i32) {
	println!("got a: {}", x);
}

// no semicolon in the return
// we have to specify return type
// this doesn't change 'x'
fn double(x: i32) -> i32 {
	x * 2
}

fn main() {
	let mut x = 5; 		// in rust, we use 'let' to do assignment
						// unless we specify 'mut' for mutable, things are const
	x = 10;

	let y: i32 = 5;		// specify 32-bit integer

    println!("Value of x: {}", x);	// this is how we print variables in rust

    // if statement
    if x == 10 {
    	println!("this syntax is weird");
    } else if y == 5 {
    	println!("trippy {} {}", x, y)
    }

    something(x);
    double(x);		// doesnt change x
    something(x);
    x = double(x);	// need to assign
    something(x);

    let list = "this is a string";

    println!("{}", list);

    // cannot concat with '+' operator
    //    let list2 = list + " with concatenation";

    let mut vec = vec![1, 2, 3];
    for i in 0..vec.len() {
    	println!("{}", vec[i]);
    }

    //    println!("{}", list2);

    // define stream variable
    let stdin = io::stdin();

    // while io::stdin() is defined in std::io,
    // the other functionalities are in prelude

    // get the input
    println!("Say something: ");

    // can't just do this
    // let input = stdin.read_line();

    //let input = stdin.read_line().ok();

    // the lock itself isn't good enough
    // let input = stdin.lock();

    let mut input : String = "";
    stdin.lock().read_line(input);

    // need to unwrap the data
    //println!("You said: {}", input.unwrap());

    // prints every line as it's typed in
    // delimited by \n
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }

}