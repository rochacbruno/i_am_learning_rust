#[macro_use]
extern crate sugar;

use sugar::*;

fn main() {


    // vec of boxed value
    let vb1 = vec_box![1, 2, 3];

    // list comprehension
    let vb2 = c![Box::new(i), for i in 1..4];

    // hashmap construction
    let hm1 = hashmap!{
        1 => 2,
        2 => 3,
        3 => 4,
    };

    // hashmap comprehension
    let hm2 = c![i => i + 1, for i in 1..4];

    let _ = max!(1, 2, 3);
    let _ = min!(1, 2, 3);

    let num = 1000;
    
    if cmp!(1, < num, < 3) {
        println!("hello world");
    }

}
