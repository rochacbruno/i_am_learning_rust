#![allow(dead_code)]
fn main() {
    // Enums are used to create a type to represent variations
    // enums can hold unit-like variants

    enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    let player_direction = Direction::Down;

    match player_direction {
        Direction::Up => println!("We are heading up!"),
        Direction::Down => println!("We are falling down!"),
        Direction::Left => println!("We are turning left!"),
        Direction::Right => println!("We are turning right!")
    }

    // enums can hold structs
    enum UserEvent {
        KeyPress(char), // tuple like
        Message(String),
        Click {x: i64, y: i64}  // structure like
    }

    let event = UserEvent::KeyPress('x');
    // let event = UserEvent::Message("Hello".to_string());
    // let event = UserEvent::Click {x: 45, y: 67};

    match event {
        UserEvent::KeyPress(k) => println!("Key pressed  {}", k),
        UserEvent::Message(s) => println!("Message content: {}", s),
        UserEvent::Click {x, y} => {
            println!("User cliked at x={}, y={}.", x, y);
        }
    }

    // A very important enum in Rust is Option
    //     The Option<T> enum has two variants:
    // None, to indicate failure or lack of value, and
    // Some(value), a tuple struct that wraps a value with type T
    fn division(dividend: i32, divisor: i32) -> Option<i32> {
        if divisor  == 0 { None } else { Some(dividend / divisor) }
    }

    match division(4, 3) {
        None => println!("Division Error"),
        Some(result) => println!("Result of division {}", result)
    }

    // Explain the `unwrap`
    let result = division(4, 3);
    println!("Wrapped result: {:?}", result);

    let result = division(4, 0);
    println!("Wrapped result: {:?}", result);

    // unwrap on Some extracts the value
    let result = division(4, 3);
    println!("Unwrapped result: {:?}", result.unwrap());

    // Unwrapping a `None` variant will `panic!`
    let result = division(4, 0);
    // println!("Wrapped result: {:?}", result.unwrap());
    match result {
        None => {},
        Some(x) => {println!("result is {}", x)}
    }
    // println!("Wrapped OR result: {:?}", result.unwrap_or(0));

    let name = String::from("Bruno");
    // Try to read the char at index 5
    println!("Char at index 5 is: {}", match name.chars().nth(5) {
        Some(c) => c.to_string(),
        None => "No char found in index 5".to_string()
    });


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

    // You can `impl` methods on Enums
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
    println!("Is today weekday? {}", today.is_weekday());
    let yesterday = Day::Fri;
    println!("Was yesterday weekday? {}", yesterday.is_weekday());



    // Result is smarter than Option and allows to express why the operation 
    // failed. 
    // Result<OK(T), Err(why)> enum has 2 variants Ok(value: T), and Err(why: E)
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    fn smarter_division(dividend: i32, divisor: i32) -> Result<i32, MathError> {
        if divisor == 0 { 
            Err(MathError::DivisionByZero)
        } else { 
            Ok(dividend / divisor) 
        }
    }

    // We can get the result wrapped by Result type
    let result = smarter_division(4, 0);
    println!("wrapped smart error is: {:?}", result);

    // we can match over a Result
    match smarter_division(4, 2) {
        Err(why) => panic!("{:?}", why),
        // unwrapped here
        Ok(result) => println!("result using match over Result is {}", result)
    }

    // the .unwrap method is a shortcut to the above match
    // it panics on Err
    let result = smarter_division(4, 3);
    println!("Result using unwrap: {:?}", result.unwrap());

    let result = smarter_division(4, 2);
    println!("Result using expect: {:?}", result.expect("Division by 0"));

    // Using the ? is a shortcut to the complete match on Result
    // only works inside functions that returns Result
    fn divide_by_zero(dividend: i32) -> Result<i32, MathError> {
        // common way to propagate the error is repetitive
        // match smarter_division(dividend, 0) {
        //     Ok(result) => Ok(result),
        //     Err(e) => Err(e)
        // }
 
        // the deprecated try! macro
        // let result = try!(smarter_division(dividend, 0));
        // Ok(result)

        // better way using ?
        let result = smarter_division(dividend, 0)?;
        Ok(result)
 
    }

    println!("divide 5 by 0 {:?}", divide_by_zero(5));

}
