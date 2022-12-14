// https://www.youtube.com/watch?v=VFIOSWy93H0

fn ownership_1(){
    // Ownership Rules
    // 1. Each value in Rust has a variable that’s called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    // s is not valid here, because it's not yet declared
    {
        let s2 = "hello"; // s2 is valid and fixed string literal
        let s = String::from("hello"); // s is valid from this point forward
        // do stuff with s
    } // s is dropped here, and no longer valid
}

fn ownership_2(){
    let x = 5;
    let y = x; // Copy
    println!("x = {}, y = {}", x, y); // Both of the values will be print out as 5 since they copy it
    // Intergers are known size which does not require to transfer the ownership (same as other fixed size types)
}

fn ownership_3(){
    let s1 = String::from("Hello World");
    let s2 = s1; // Move (instead of copy)
    // https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move
    // println!("s1 = {}, s2 = {}", s1, s2); // Error: value borrowed here after move
    println!("s2 = {}", s2); // s2 is valid
}

fn ownership_4(){
    let s1 = String::from("Hello World");
    let s2 = s1.clone(); // Deep copy
    println!("s1 = {}, s2 = {}", s1, s2); // Both of the values will be print out as "Hello World" since they copy it
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {
    // gives_ownership will move its return
    // value into the function that calls it

    let some_string = String::from("New World"); // some_string comes into scope

    some_string                              // some_string is returned and
                                            // moves out to the calling
                                            // function
}

fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize){
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn calculate_length_2(s: &String) -> usize{
    s.len()
}

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }


fn change_with_mut(some_string: &mut String) {
    some_string.push_str(", world");
}




fn main() {
    ownership_1();
    ownership_2();
    ownership_3();
    ownership_4();

    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // println!("{}", s); // Error: value borrowed here after move

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    
    println!("{}", x);

    let s = gives_ownership();         // gives_ownership moves its return
                                        // value into s

    let s2 = String::from("Kenneth Kang");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3

    let s4 = String::from("Hello World, Kenneth Kang");

    let s4 = takes_and_gives_back(s4);

    // println!("s = {}, s2 = {}, s3 = {}", s, s2, s3); // Error: value borrowed here after move (for s2)

    println!("s = {}, s3 = {}", s, s3);

    println!("s4 = {}", s4);

    let (s5, len) = calculate_length(s4);

    println!("The length of '{}' is {}.", s5, len); // This can be for references rather than transforing ownership multiple times

    let s6 = String::from("What is this reference idea?");

    let len = calculate_length_2(&s6); // Reference is the same idea of borrowing rather than taking control

    println!("The length of '{}' is {}.", s6, len);

    let s7 = String::from("Hello World");
    
    // change(&s7); // This is impossible because we cannot change the value of the reference

    let mut s8 = String::from("Hello");

    change_with_mut(&mut s8); // But mutable reference is possible

    println!("s8 = {}", s8);



}
