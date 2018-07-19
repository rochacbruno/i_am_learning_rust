#![allow(dead_code)]
// The above allow is to avoid warnings about unused code and variations

// all the code is included inside main fn to simplify the interactive
// execution

fn main() {
    println!("--- Enums ---");
    // Enums
    // are used to create an atomic type to represent variations
    // enums can hold unit-like variants:

    enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    // As enum variants are `static` we access it with `::`
    let player_direction = Direction::Down;

    // Enums are useful to `match` on variables (works like a switch - case)
    match player_direction {
        Direction::Up => println!("We are heading up!"),
        Direction::Down => println!("We are falling down!"),
        Direction::Left => println!("We are turning left!"),
        Direction::Right => println!("We are turning right!")
    }

    // Enums can also hold more complex struct variants
    enum UserEvent {
        KeyPress(char), // tuple like holding a char
        Message(String),  // tuple like holding a string
        Click {x: i64, y: i64}  // structure like holding pair of i64
    }

    // So we can use each of the variants to define and match
    let event_one = UserEvent::KeyPress('x');
    let event_two = UserEvent::Message("Hello".to_string());
    let event_three = UserEvent::Click {x: 45, y: 67};

    for event in vec![event_one, event_two, event_three] {
        match event {
            UserEvent::KeyPress(k) => println!("Key pressed  {}", k),
            UserEvent::Message(s) => println!("Message content: {}", s),
            UserEvent::Click {x, y} => {
                println!("User cliked at x={}, y={}.", x, y);
            }
        }
    }

    println!("--- Option ---");
    // Option
    // A very important enum in Rust is Option
    // The Option<T> enum has two variants:
    // None, to indicate failure or lack of value, and
    // Some(value), a tuple struct that wraps a value with type T
    fn division(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor == 0 { None } else { Some(dividend / divisor) }
    }

    // Option values are `wrapped` as it comes inside `Some` struct.
    let result = division(4, 2);
    println!("Division wrapped Some result: {:?}", result);  // Some(2)

    // Also works for the None variant of the Option enum
    let result = division(4, 0);
    println!("Division wrapped None result: {:?}", result); // None

    // To get the proper return of the `division` we need to match it
    let result = division(4, 0);  // None 
    match result {
        None => {},  // Empty block does nothing
        Some(x) => {println!("Division matched result is {}", x)}
    }

    // We can also match for None|Some directly in the Option result
    match division(4, 2) {
        None => println!("Division Error"),
        Some(result) => println!("Division matched result is {}", result)
    }

    // There is a shortcut to the above match: `unwrap`
    // unwrap on Some variant extracts the value inside it
    let result = division(4, 2);
    println!("Division unwrapped result: {:?}", result.unwrap()); // 2

    let dividend = 4;
    let result = division(dividend, 0);

    // Unwrapping a `None` variant will `panic!`
    // println!("Wrapped result: {:?}", result.unwrap());

    // .expect is used to customize the panic message
    // println!("Division expect: {:?}", result.expect("Oh My God an error!!!"));

    // _or and _or_else wotks the same as `unwrap`
    println!("Division unwrap_or: {:?}", result.unwrap_or(0));
    println!("Division unwrap_or_else: {:?}", result.unwrap_or_else(|| dividend / 2 ));

    // Real use cases for matching over an Option

    let name = String::from("Bruno");
    // Try to read the char at index 5
    println!("Char at index 5 is: {}", match name.chars().nth(5) { // Returns Option
        Some(c) => c.to_string(),
        None => "No char found in index 5".to_string()
    });

    // A function to query occupation by person name
    fn get_occupation(name: &str) -> Option<&str> {
        match name {
            "Bruno" => Some("Software Quality Engineer"),
            "Karla" => Some("Photographer"),
            _ => None
        }
    }

    for name in vec!["Bruno", "Karla", "John"] {
        println!("{} is: {}", name, match get_occupation(name) {
            Some(o) => o,
            None => "No ocupation"
        })
    }
    // equivalent of above
    for name in vec!["Bruno", "Karla", "John"] {
        println!("{} is: {}", name, get_occupation(name).unwrap_or("No occupation"));
    }

    // Enums can have methods
    enum Day {Mon, Tue, Wed, Thu, Fri, Sat, Sun}
    
    impl Day {
        fn is_weekday(&self) -> bool {
            match self {
                &Day::Sat | &Day::Sun => false,
                _ => true
            }
        }
    }

    let today = Day::Sat;
    // methods are not `static` so called by `.`
    println!("Is today weekday? {}", today.is_weekday());
    let yesterday = Day::Fri;
    println!("Was yesterday weekday? {}", yesterday.is_weekday());


    println!("--- Result ---");
    // Result
    // Result is smarter than Option and allows to express why the operation 
    // failed. 
    /// It is described as "Recoverable Error handling"
    // Result<Ok(T), Err(E)> enum has 2 variants Ok(value: T), and Err(reason: E)

    // First we define an Enum of possible errors
    #[derive(Debug)]  // See `derive traits`
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    // Now our function returns an Result instead of Option
    fn smart_division(dividend: i32, divisor: i32) -> Result<i32, MathError> {
        if divisor == 0 { 
            Err(MathError::DivisionByZero)
        } else { 
            Ok(dividend / divisor) 
        }
    }

    // We can get the result `wrapped` by Result type
    let result = smart_division(4, 0);
    println!("smart_division wrapped: {:?}", result);

    // we can match over Result variants
    match smart_division(4, 2) {
        Err(reason) => panic!("Oh my God another error! {:?}", reason),
        // automatically unwrapped here
        Ok(result) => println!("smart_division match result {}", result)
    }

    // the .unwrap method is a shortcut to the above match
    // it panics on Err
    let result = smart_division(4, 3);
    println!("smart_division unwrap: {:?}", result.unwrap());

    // .expect helps to add custom message to panic
    let result = smart_division(4, 2);
    println!("smart_division expect: {:?}", result.expect("Division by 0"));

    println!("--- ? ---");
    // Propagating Errors
    fn divide_by_zero(dividend: i32) -> Result<i32, MathError> {
        // common way to propagate the error is repetitive
        // match smart_division(dividend, 0) {
        //     Ok(result) => Ok(result),
        //     Err(e) => Err(e)
        // }
 
        // the deprecated try! macro
        // let result = try!(smart_division(dividend, 0));
        // Ok(result)

        // Using the ? is a shortcut to the complete match on Result
        // only works inside functions that returns Result
        let result = smart_division(dividend, 0)?;
        Ok(result)
 
    }

    println!("Error propagation on 5 by 0: {:?}", divide_by_zero(5));

}
