fn main() {
    // Enums are used to create a type to represent variations
    // enums can hold unit-like variants

    // `allow` required to silence warnings because only
    // one variant is used.
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    enum UserEvent {
        KeyPress(char), // tuple like
        Data(String),
        Click{x: i64, y: i64}  // structure like
    }

    let event = UserEvent::KeyPress('x');
    // let event = UserEvent::Data("Hello".to_string());
    // let event = UserEvent::Click {x: 45, y: 67};

    match event {
        UserEvent::KeyPress(k) => println!("Key pressed  {}", k),
        UserEvent::Data(s) => println!("Data content: {}", s),
        UserEvent::Click {x, y} => {
            println!("User cliked at x={}, y={}.", x, y);
        }
    }

    // A very important enum in Rust is Option
    //     The Option<T> enum has two variants:
    // None, to indicate failure or lack of value, and
    // Some(value), a tuple struct that wraps a value with type T

    #[allow(dead_code)]
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


    // Result is smarter than Option and allows to express why the operation 
    // failed. 
    // Result<T, E> enum has 2 variants Ok(value: T), and Err(why: E)

    #[allow(dead_code)]
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NonPositiveLogarithm,
        NegativeSquareRoot,
    }

    #[allow(dead_code)]
    fn smarter_division(dividend: i32, divisor: i32) -> Result<i32, MathError> {
        if divisor == 0 { 
            Err(MathError::DivisionByZero)
        } else { 
            Ok(dividend / divisor) 
        }
    }

    match smarter_division(4, 2) {
        Err(why) => panic!("{:?}", why),
        // unwrapped here
        Ok(result) => println!("smarter result is {}", result)
    }

    let result = smarter_division(4, 0);
    println!("wrapped smart error is: {:?}", result);

    let result = smarter_division(4, 4);
    println!("Unwrapped smart result is: {:?}", result.unwrap());

    // panics on unwrap error
    // let result = smarter_division(4, 0);
    // println!("Unwrapped smart panic is: {:?}", result.unwrap());

    // expect and ?
    // let result = smarter_division(4, 3);
    // println!("Unwrapped smart panic is: {:?}", result);
}
