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
}



fn main() {
    ownership_1();
    ownership_2();
}
