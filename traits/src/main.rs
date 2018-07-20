#![allow(dead_code)]

fn main() {
    println!("--- Traits ---");

    // Traits defined behavior for types like struct and enum
    
    // Compiler provides some basic common traits via [derive] attribute 
    // https://doc.rust-lang.org/rust-by-example/trait/derive.html

    // Comparison traits: `Eq, PartialEq, Ord, PartialOrd`
    // `Clone`, to create T from &T via a copy.
    // `Copy`, to give a type 'copy semantics' instead of 'move semantics'
    // `Hash`, to compute a hash from &T.
    // `Default`, to create an empty instance of a data type.
    // `Debug `, to format a value using the {:?} or {:#?} formatters.

    // #[derive(Debug)]
    #[derive(Debug, Copy, Clone)]  // LOOK ABOVE
    struct Person {
        id: u8
    }

    let person = Person {id: 100};
    println!("Person can be debugged: {:?}", person);

    println!("--- Interfaces ---");
    // Traits are just like Interfaces
    // or also known as ABC in POO languages (Abstract Base Classes)
    // Trait (interface) with a `Not Implemented` method
    trait Deletable {
        fn delete(&self) -> bool;  // implementors should define this method
    }

    impl Deletable for Person {
        fn delete(&self) -> bool {
            println!("Deleting person...");
            true
        }
    }
    
    person.delete();

    println!("--- Default method implementations ---");

    // Trait can define a default implementation
    // that can be optionally overwritten
    trait Elf {
        fn translate_to_elvish(&self, word: &str) -> String {
            format!("{} is `jsafsd sadibfuysdf` in Elvish", word)
        }
    }

    impl Elf for Person {};  // empty impl stands with the default

    println!("Person says: {}", person.translate_to_elvish("Hello"));


    println!("--- Into<String> ---");
    // A very common trait used in Rust is `ToString`
    // which enables the use of `.to_string` method
    // and can be used together with `Into<String>` trait bound

    impl ToString for Person {
        fn to_string(&self) -> String {
            format!("person with the id {}", self.id)
        }
    }

    println!("This is {}", person.to_string());
    

    println!("--- Copy and Clone ---");
    // Traits can be used to change the move semantics
    // what happens if I want to copy the person?

    // assuming Person do not derives Copy and Clone trait
    let _otherperson = person;  // this moves ownership
    person.to_string();  // this is not allowed as person moved
   

    // Now once person derives from Copy, Clone
    let mut otherperson = person;  // now this copies person
    otherperson.id = person.id + 1;
    println!("This is original {:?}", person);
    println!("This is copied {:?}", otherperson);


    println!("--- Ordering ---");
    // Other useful traits are PartialEq and PartialOrd
    let john = Person { id: 10 };
    let mike = Person { id: 12 };

    // Comparing two Persons errors with 
    //  an implementation of `std::cmp::PartialOrd` might be missing for `main::Person`
    if john > mike {
        println!("John wins");
    }

    // Lets make john and mike comparable by implementing PartialEq and PartialOrd
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

    if mike > john {
        println!("Mike wins");
    }
    if john <= mike {
        println!("Mike wins again");
    }

    println!("--- Operator overriding ---");
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

    let paul = john + mike;
    println!("John + Mike is {:?}", paul);

     println!("--- Trait bounds ---");
    // Trait bounds
    // we can bind generic types to function definitions
    // lets make a function which takes as argument T
    // only if T implements the Elf trait
    fn speak_in_elvish<T: Elf>(object: &T) -> String {
        object.translate_to_elvish("Hoof Hoof!")
    }

    struct Animal {id: u8 };
    let rex = Animal {id: 9};

    // errors with 
    // the trait `main::Elf` is not implemented for `main::Animal`
    // speak_in_elvish(rex);  

    // unless
    impl Elf for Animal {};

    println!("Rex says {}", speak_in_elvish(&rex));  

    println!("--- Conditional Trait Bounds ---");
    // Conditional trait bounds
    // Implement specific trait to all structs that also implements other trait
    // Example: I want the `scream_in_elvish` implemented for everyone who implements Elf

    trait ScreamElf {
        fn scream_in_elvish(&self, word: &str) -> String {
            format!("{} is ASJNDASD ASDASD in elvish", word)
        }
    }

    // all structs that implements `Elf` will automatically implement ScreamElf
    impl<T: Elf> ScreamElf for T {};

    println!("Rex screams {}", rex.scream_in_elvish("Hoof Hoof"));
    println!("Paul screams {}", paul.scream_in_elvish("Hello"));
    println!("Mike screams {}", mike.scream_in_elvish("Hello"));
    println!("John screams {}", john.scream_in_elvish("Hello"));

   println!("--- Enum traits ---");
   // Traits can also be implemented on Enum type
   enum Animals {Cat, Dog, Bird};
   impl Elf for Animals {};
   println!("Cat says {}", Animals::Cat.translate_to_elvish("meow"));
}
