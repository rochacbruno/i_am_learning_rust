#![allow(dead_code)]

fn main() {
    println!("Hello World");

    println!("-- Lets talk about structs");
    // A Struct is a composite data type 
    // There are 3 types of struct in Rust. unit, tuple and data.

    // # Unit structs
    struct Empty; // Atomic symbol with no data useful for generics and impls

    // # Tuple structs
    struct RGBColor(u8, u8, u8);  // A struct holding 3 indexed fields
    let blue = RGBColor(0, 0 ,255);

    // fields are accessible by index number
    println!("R:{} G:{} B:{}", blue.0, blue.1, blue.2);

    // we can 'deconstruct' struct variables
    let RGBColor(r, g, b) = blue;
    println!("R:{} G:{} B:{}", r, g, b);

    // # Data structs
    struct CMYKColor {cyan: u8, magenta: u8, yellow: u8, black: u8};
    let gray = CMYKColor {cyan: 30, magenta: 30, yellow: 30, black: 30};
    println!("C:{} M:{} Y:{} K:{}", gray.cyan, gray.magenta, gray.yellow, gray.black);

    // Structs can be used as struct fields
    #[derive(Debug)]  // derive the trait Debug to allow printing with {:?}
    struct Point(i32, i32);

    #[derive(Debug)]
    struct Rectangle {p1: Point, p2: Point};

    let rect = Rectangle {p1: Point(3, 5), p2: Point(10, 12)};
    println!("{:#?}", rect);

    println!("-- Lets talk about Enums");
    // Enums are a group of structs to represent variants

    // # unit like enums
    // as enum variants are `static` we access it using `::` notation
    enum Direction {Up, Down, Left, Right};
    let plane_direction = Direction::Down;  // the eagle is landing
    
    enum LogLevel {Debug, Info, Warning, Error, Critical};
    let logger_level = LogLevel::Debug;

    // Enums can also hold more complex struct variants
    enum UserEvent {
        Ping,                   // unit 
        KeyPress(char),         // tuple
        Message(String, i32),    // tuple
        Click {x: i32, y: i32}  // data
    }

    let event_one = UserEvent::Ping;
    let event_two = UserEvent::KeyPress('x');
    let event_three = UserEvent::Message("Hello".to_string(), 5);
    let event_four = UserEvent::Click {x: 45, y: 67};

    println!("-- Lets talk about matches");

    // Match on Enum
     let player_direction = Direction::Down;

    // Enums are useful to `match` on variables (works like a switch - case)
    match player_direction {
        Direction::Up => println!("heading up!"),
        Direction::Down => println!("falling down!"),
        Direction::Left => println!("turning left!"),
        Direction::Right => println!("turning right!")
    }

    for event in vec![event_one, event_two, event_three, event_four] {
        match event {
            UserEvent::Ping => println!("User Pinged"),  // try to remove this
            UserEvent::KeyPress(key) => println!("Key pressed {}", key),
            UserEvent::Message(msg, _) => println!("Message content: {}", msg),
            UserEvent::Click {x, y} => {
                println!("User cliked at x={}, y={}.", x, y);
            },
            // _ => {} // this catches all other cases...
        }
    }

    // Match on struct
    let point = Point(0, 5);
    match point {
        Point(x, _) if x > 0 => println!("x is greater than 0"),
        Point(_, y) if y > 0 => println!("y is greater than 0"),
        _ => println!("x and y are 0")
    }

    println!("-- Lets talk about `Option`");
    // Option is an enum with two variants 
    // None to indicate failure or lack of value
    // Some(value) to indicate the presence of value of type T
    // It is a common practice to return function results
    // wrapped in the Option enum

    // Wrapped Option

    fn division(dividend: i32, divisor: i32) -> Option<i32> {
        // disclaimer don't judge this function implementation.
        if divisor == 0 { None } else { Some(dividend / divisor) }
    } 

    let result = division(20, 2);
    println!("Division Some result is wrapped: {:?}", result);

    let result = division(4, 0);
    println!("Division None result is wrapped: {:?}", result);

    // How to unwrap it and get the `i32` value?
    let result = division(4, 2);
    match result {  // we can call `match division(x, y)` directly
        Some(value) => println!("Division matched result is: {}", value),
        None => {} // do nothing
    }

    // There is a handy `unwrap` shortcut to the above match!
    // Haters gonna hate! :)
    let result = division(20, 4);
    println!("Division unwrapped result is {}", result.unwrap());

    // In case of `None` it panics! so we should use `unwrap_or`
    let div = 2;
    let result = division(div, 0);
    println!("Division unwrapped or {}", result.unwrap_or(0));
    println!("Division unwrapped or else {}", result.unwrap_or_else(|| div / 1));
    // Ok this is so wrong...

    // better to panic with a custom message
    // println!("Division panics: {}", result.expect("Oh My God! an error"))

    println!("-- Lets talk about Result");
    // Result is an enum with two variants
    // Ok(value) indicates the presence of value of type T
    // Err(reason) indicates error occured for the reason of type E
    // T and E are `Generics` so any struct is allowed
    // Result is used for "Recoverable Error Handling"

    // First we need a set of reasons
    #[derive(Debug)]
    pub enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
        OtherStrangeMathProblem
    }

    // Now the division is smarter as it can specify error types
    fn smart_division(dividend: i32, divisor: i32) -> Result<i32, MathError> {
        if divisor == 0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(dividend / divisor)
        }
    }

    // lets get the division result wrapped by a Result
    let result = smart_division(20, 2);
    println!("Result wrapped {:?}", result);

    // How to unwrap that and get the actual i32?
    // NOTE that we can assign the match expression
    let result = match smart_division(20, 2) {
        Err(reason) => panic!("Oh my god another error {:?}", reason),
        Ok(value) => value
    };
    println!("Unwrapped by the match {}", result);

    // Oh yeah we can again use the `unwrap`, `unwrap_or`, `expect` etc..
    let result = smart_division(20, 5).expect("Oh my god another error");
    println!("Unwrapped Result {}", result);

    println!("-- Lets talk about `impl` a.k.a methods");
    // Types in Rust can have `method` implementations `impl`
    // Structs and Enums can have methods attached

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
    println!("Is saturday weekday? {}", today.is_weekday());
    let yesterday = Day::Fri;
    println!("Was friday weekday? {}", yesterday.is_weekday());

    // Structs can have methods
    struct Area {w: u32, h: u32}
    impl Area {
        fn is_square(&self) -> bool {
            self.w == self.h
        }
    }   
    
    let room = Area {w: 30, h: 30};
    println!("The room is squared: {}", room.is_square());

    // You can have multiple `impl` blocks, on demand
    impl Area {
        fn total(&self) -> u32 {
            self.w * self.h
        }
    }
    println!("The room total area is: {}", room.total());

    println!("-- Lets talk about Traits (a.k.a interfaces)");

    // Traits defines behavior for types
    
    // Compiler provides some basic common traits via [derive] attribute 
    // https://doc.rust-lang.org/rust-by-example/trait/derive.html

    // Comparison traits: `Eq, PartialEq, Ord, PartialOrd`
    // `Clone`, to create T from &T via a copy.
    // `Copy`, to give a type 'copy semantics' instead of 'move semantics'
    // `Hash`, to compute a hash from &T.
    // `Default`, to create an empty instance of a data type.
    // `Debug `, to format a value using the {:?} or {:#?} formatters

    #[derive(Debug)]
    struct Person {
        id: u8
    }
    // impl std::fmt::Debug for Person {};

    let clark = Person {id: 50};
    println!("Person can be debugged: {:?}", clark);

    // Traits works like "interface" Lso known as ABC in POO languages
    // A trait with a `not implemented` method
    trait Deletable {
        fn delete(&self) -> bool;
    }

    impl Deletable for Person {
        fn delete(&self) -> bool {
            println!("DELETE FROM PERSON WHERE ID={}", self.id);
            true
        }
    }

    clark.delete();

    // Traits with default implementations

    trait SuperHero {
        fn fly(&self, location: &str) -> String {
            format!("flying to {}", location)
        }
    }

    impl SuperHero for Person {};

    // starting here I can call `.fly` in any instance of type Person
    println!("clark is {}", clark.fly("Metropolis"));

    // # Traits of comparision
    let peter = Person {id: 100};

    // Comparing two Persons errors with 
    //  an implementation of `std::cmp::PartialOrd` might be missing for `main::Person`
    // if peter > clark {
    //     println!("peter wins");
    // }

    // Lets make peter and clark comparable by implementing PartialEq and PartialOrd
    impl PartialEq for Person {
        fn eq(&self, other: &Person) -> bool {
            self.id == other.id
        }
    }
    use std::cmp::Ordering;
    impl PartialOrd for Person {
        fn partial_cmp(&self, other: &Person) -> Option<Ordering> {
            Some(self.id.cmp(&other.id))
        }
    }

    if clark < peter {
        println!("peter wins");
    }
    if peter >= clark {
        println!("peter wins again");
    }
    
    // # Operator overrride
    // how to override `+` operator using traits
    // when using `x + y` what Rust does is calling `x.add(y)`
    // very similar to Python's __add__ method.
    use std::ops;
    impl ops::Add<Person> for Person {
        type Output = Person;
        fn add(self, rhs: Person) -> Self::Output {
            Person {id: self.id + rhs.id}
        }
    }

    let superspider = peter + clark;
    println!("peter + clark is {:?}", superspider);

    // # Trait bounds
    // we can add generic type constraints to functions 
    fn kill_flying_monster<T: SuperHero>(object: &T) {
        println!("{}, killing the monster", object.fly("monster location"));
    }

    // # Developer can't kill a flying monster

    struct Developer {id: i32};
    let bruno = Developer {id: 22};
    // kill_flying_monster(&bruno);

    // person implements SuperHero and can kill a flying monster
    let wayne = Person {id: 22};
    kill_flying_monster(&wayne);

    // # Conditional Traits
    // What if we want to give time travel power to all SuperHeroes?

    trait TimeMachine {
        fn travel_to_year(&self, year: &str) {
            println!("traveling to year {}", year);
        } 
    }

    // all types that implements `SuperHero` will automatically implement TimeMachine
    impl<T: SuperHero> TimeMachine for T {};

    wayne.travel_to_year("1910");
    superspider.travel_to_year("2089");

    // Traits can also be applied to enums
    enum Animal {Cat, Dog, Bird};
    impl SuperHero for Animal {};
    println!("cat is {}", Animal::Cat.fly("catland"));
    kill_flying_monster(&Animal::Dog);
    Animal::Bird.travel_to_year("1900");

    println!("-- Lets talk about the Builder Pattern");

    // It is very common to create new objects
    // with a syntax like:
    // let tomato = Food::new(color="red")
    // let celery = Food::new(color="green")

    struct Food {color: String}
    impl Food {
        pub fn new(color: &str) -> Self {
            Self {color: color.to_string()}
        }
    }
    
    // With the above impl we can create new objects like this
    let tomato = Food::new("red");
    let celery = Food::new("green");

    println!("Tomato is {}", tomato.color);
    println!("Celery is {}", celery.color);

}
