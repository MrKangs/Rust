fn variables() {

    // Mutability Method 1
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    
    // const variable: very different than normal variable: 
    // cannot not have a mut method
    // cannot be define from function return value
    // must declare data type
    // numerical literacy
    const COUNTER: u32 = 100_000;

    // Mutability Method 2
    let newx = 5;
    println!("The value of x is {}", newx);
    let newx = "Six";
    println!("The value of x is {}", newx);

    
}

fn datatypes() {
    // Scaler

    // Integers: i8, i16, i32, o64, i128, isize, u8, u16, u32, u64, u128, usize
        // i = signed, u = unsigned
    
    let a = 5; // i32 is default
    println!("{}",a);

    let b = 98_222; // 98222 Decimal
    println!("{}",b);

    let c = 0xff; // Hex
    println!("{}",c);

    let d = 0o77; // Octal
    println!("{}",d);

    let e = 0b1111_0000; // Binary
    println!("{}",e);

    let f = b'A'; // Bytes (u8 only)
    println!("{}",f);

    // Floating Point numbers

    let g = 2.5; // f64 is default
    println!("{}",g);

    let h: f32 = 3.3;
    println!("{}", h);

    // Addition
    let sum = 5 + 10;
    println!("{}", sum);

    // Subtraction
    let difference = 95.5-32.1;
    println!("{}", difference);

    // product
    let product = 95.5 * 32.1;
    println!("{}", product);

    // quotient
    let quotient = 95.5 / 32.1;
    println!("{}", quotient);

    // remainder
    let remainder = 43 % 5;
    println!("{}", remainder);

    // Math operations must be in the same data type

    // Booleans

    let t = true;
    let no = false;

    // Characters

    let character = 'H';
    let newchar = 'h';
    let interesting = '🪳';
    println!("{}", interesting);

    // Compound Type

    // Tuples
    let tup = ("Let's get Rusty!", 100_000);
    let (channel, sub_count) = tup;
    let subCount = tup.1;
    println!("{}", subCount);

    // Array
    // Cannot be dynamic: Vector will do the job
    let error_codes = [200,404,500];
    let not_found = error_codes[1];

    // Create an Array that has 8 spaces with a value of 0
    let byte = [0;8];

    // Error will be occur when complie for out of index/bounds for the array

}

fn my_function(){
    println!("Another function");
    // The function name must be lowercase and word in between will have a space
}

fn my_function_2(x:i32, y:i32) {
    // For parameter, need to declare the type of variable that is passing in
    
    // print is a statement, not an expression
    println!("x = {}",x);
    println!("y = {}",y);
}

fn my_function_3(x:i32, y:i32) -> i32{
    // Need to declare what is the return type as well
    // There are two ways to return values

    // return x + y;
    x + y
}

fn control_flow(){
    let number = 5;

    if number < 10 {
        // The comparison or condition must be a boolean value, not just a value itself: that will cause an error
        println!("first condition was true");
    } else if number < 22{
        println!("second condition was true");
    } else {
        println!("condition was false");
    }

    let condition = false;
    let number = if condition {5} else {6};
    // If condition satisfy, the 5 otherwise 6
    println!("{}", number);

}

fn loops(){
    // There are three types of loops
    loop {
        println!("First Loop");
        break;
    }
    // This first loop will never end until it hits a break statement
    // You can get a loop return value by doing the following    
    
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10{
            break counter;
        }
    };
    println!("result: {}", result);
    
    // While loop
    let mut number = 3;
    while number != 0{
        println!("{}", number);
        number -= 1;
    }
    println!("Blass off!");

    // Last type of loop is the for loop
    // Only can iterate over arrays or range
    let a = [10, 20, 30, 40 ,50];

    // iter() function will iterate every value of a
    for element in a.iter(){
        println!("The value is: {}", element);
    }

    // .. indicates the rante between left value to the right value
    for number in 1..4 {
        println!("{}!", number);
    }
}

fn main() {
    variables();
    datatypes();
    my_function();
    my_function_2(10, 20);
    let sum = my_function_3(10, 20);
    println!("{}", sum);
    control_flow();
    loops();
}
