//! This is the first example in TRPL book
//! I added my notes to remember things here
//! module explanation comments starts with `//!` so it goes to the docs

// import external crate (library is called crate in rust)
extern crate rand;

// import objects from std crates to local namespace
use std::io;
use std::cmp::Ordering;
// already imported this external crate, now it sets the local visibility
// this is also needed to bring crate's `Traits` in to the scope
use rand::Rng;


/// # What is main?
/// ```
/// fn main() {...}
/// ```
/// The `main` function is the point of entry for rust binaries
/// when the program is compiled a binary is generated and main is
/// the invoked function when binary is executed
///
/// > And yes we can write comments starting with `///` which means
/// > this is a `documentation` of the object below
/// > This comments accepts MARKDOWN syntax and is used to generate docs.
/// - markdown is amazing
/// - markdown is love
/// - markdown is better than .rst
/// end.
fn main() {
    // println! is a macro, that is why it ends with `!`
    // A macro differs from a function because it is pre-evaluated
    // in pre-compilation and its code can be replaced by actual code
    // this gives meta-programming powers to rust.
    println!("Guess the number!");

    // let defines variables
    // all attributions in Rust is a `pattern match`
    // we use `struct::static_method()` to access static methods
    // then we use `instance.method()` to access instance methods
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // loop is an infinite loop
    loop {

        // a bare string as "hello" is a stack/immutable value
        // the heap/dynamic string is `String` and to make it
        // immutable we need to specify mutability `let mut`.
        // and we are shadowing the variable `guess` for each loop.
        let mut guess = String::new();

        // some methods return a `Result` struct the result is a common
        // return structure in Rust, it allows to use pattern match and also
        // exposes a `.expect()` method where we define the error message
        // in case of panic (exception).
        io::stdin().read_line(&mut guess).expect(
            "failed to read the line",
        );

        // pattern match can be used in attribution
        // there the `.parse()` method of `String` uses a pattern matching
        // to return a `u32` casted value and we can match the `Result`
        // to decide what to do in case of success `Ok` or panic `Err`
        // the flow can be returning a value as in `=> num` or an statement as
        // in `=> continue` which continues the parent loop.
        let guess: u32 = match guess.trim().parse() {
            // pattern matching again here to unpack num to num
            Ok(num) => num,
            // _ (underline) means None or 'catch all' in this case
            Err(_) => continue
        };

        // string interpolation using `{}` placeholders
        println!("You Guessed {}", guess);

        // in the following match the value `&secret_number` is passed
        // as reference as we do not want to move the real value
        // the `guess` String got the `.cmp` method provided by `Ordering` Trait
        // which we imported, if we didn't import the `Ordering` this line
        // would panic missing the Trait in scope.
        // IN this match we are mathing the `.cmp` `Result` against the
        // Ordering enum which is the same as -1(Less), 0(Equal), 1(Greater)

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => { //match flow can be a block
                println!("You win!");
                // break the parent loop.
                break;
            }
        }
    }
}
