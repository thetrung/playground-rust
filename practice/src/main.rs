
use std::io;
use std::cmp::Ordering;

use io::stdin;

fn main() {
    
    hello_world();

    println!("Square of 42 is {}", square(42));

    let array = test_array();

    let guess = array[3];

    guessing_game(guess);

    test_ownership();

    test_borrow();

    no_dangle(); // or no returning pointer outside of function scope.

    test_slice();

    println!();
}

fn hello_world(){
    // normal print
    println!("Hello, world!");
}

fn square(n: i32) -> i32 {
    if n == 0 { println!("ZERO !"); return 0;}
    else {
        // double
        return n * n;
    }
}

fn test_array() -> [i32;5] {
    // array
    let array:[i32;5] = [1,2,3,4,5];

    println!("All elements in array (iter):");
    for elem in array.iter(){
        print!(" {}", elem);
    }
    println!("");

    println!("Test access: array[2] = {}", array[2]);

    println!("Test range 1..5 reverse:");
    for number in (1..5).rev() {
        print!(" {}", number);
    }
    println!("");

    return array;
}

fn guessing_game(guess: i32) {    
    println!("== Start Guessing Game ==");
    println!("Input your number: ");
    // counting guessing times
    let mut counter = 0;

    let result = loop {
        // get input from user
        let mut input = String::new();
        stdin().read_line(&mut input).expect("Failed to get input");
        
        // without trim() it will get error for having other invalid character to parse.
        let parsed_result:i32 = input.trim().parse().expect("It's not a valid number.");

        // matching 
        match parsed_result.cmp(&guess) {
            Ordering::Less => println!("No.. It's less"),
            Ordering::Greater => println!("No... It's greater"),
            Ordering::Equal => { 
                println!("Great ! It's equal !");
                break counter; // ending loop with result
            }
        }
        counter+= 1;
    };
    println!("correct after {} loops !\n", result);

}

fn test_ownership(){
    println!("== Test Ownership ==");
    let s1 = String::from("hello");
    let s2 = s1.clone(); 
    // 
    // without clone(), 
    // s1 pointer will be "moved" into s2, and invalidated
    // so compiler will show error.
    //
    // RULES:
    // only value types of integer, float, boolean, char, 
    // and tuple of value types are COPY.
    //
    // the rest are MOVED as pointer.
    //

    println!("{}, world!\n", s1);
}

fn get_length(content: &String) -> usize{
    content.len()
}

fn test_borrow(){
    println!("== Test Borrow ==");
    let mut test_string = String::from("Hello, World !");
    println!("The length of {} = {}",test_string, get_length(&test_string));

    println!("\n== Test Mutable Reference ==");
    //
    // 1.You can have only one mutable reference in one scope.
    // 2.You can't have both mutable and immutable reference in one scope.
    //
    test_mutate_borrow(&mut test_string);
    println!("test_string after change: {}", test_string);
}

fn test_mutate_borrow(content: &mut String){
    content.push_str(" Ha ha.. nailed !");
}

//
// Return the ownership instead of reference,
// so the ownership would be moved out instead of being dropped,
// and the reference to whatever outside this scope is invalid.
//
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn test_slice(){
    //
    // Another data type that does not have ownership is the slice. 
    // Slices let you reference a contiguous sequence of elements in 
    // a collection rather than the whole collection.
    //
    // A Slice Type is like a tuple of value (usize, usize)
    //
    // Recall from the borrowing rules that if:
    // we have an immutable reference to something, 
    // we cannot also take a mutable reference.
    //
    //
    println!("\n== Test Slice Type & Syntax &[..] ==");

    // &str is an immutable reference, called "string slice"
    let my_string = String::from("hello world");
    // first_word works on slices of `String`s so we pass on a slice of it
    let word = first_word(&my_string[..]);
    println!("with mutable String, with slice syntax: {}", word);

    let my_string_literal = "hello world";
    // first_word works on slices of string literals
    let word = first_word(&my_string_literal[..]);
    println!("with immutable string literal, slice syntax: {}", word);

    // Because string literals *are* string slices already:
    let word = first_word(my_string_literal);
    println!("with immutable string literal,, without slice syntax: {}", word);
}

