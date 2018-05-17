#[macro_use(c)]
extern crate cute;

use std::collections::HashMap;


fn main() {
    let squares = c![x*x, for x in 0..10];
    let even_squares = c![x*x, for x in 0..10, if x % 2 == 0];
    let squares_hashmap = c!{key => key*key, for key in 0..10};

    // Simpe comprehension
    let v = vec![1,2,3,4];
    let v_squared = c![x*x, for x in v];

    
    //Conditional filtering
    let squares = c![x*x, for x in 0..10, if x % 2 == 0];
    assert_eq!(squares, vec![0, 4, 16, 36, 64]);

    // Nested Comprehensions
    let nested = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    let flat: Vec<usize> = c![x, for x in y, for y in nested];
    assert_eq!(flat, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);

    let nested = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    let even_flat: Vec<usize> = c![x, for x in y, for y in nested, if x % 2 == 0];
    assert_eq!(even_flat, vec![2, 4, 6, 8]);

    // Comprehensions over Iterators
    let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
    let output: Vec<i32> = c![x*2, for x in vec.iter()];
    assert_eq!(output, vec![-8, -4, 0, 4, 8]);


    let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
    let output: Vec<i32> = c![x, for x in vec.iter(), if *x >= 0i32];
    assert_eq!(output, vec![0, 2, 4]);

    // Function Application
    let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
    let output: Vec<i32> = c![x.abs(), for x in vec.iter()];
    assert_eq!(output, vec![4, 2, 0, 2, 4]);


    fn square(x:i32) -> i32 {
        x*x
    }
        
    let vec: Vec<i32> = vec![-4, -2, 0, 2, 4];
    let squares: Vec<i32> = c![square(x), for x in vec];
    assert_eq!(squares, vec![16, 4, 0, 4, 16]);

    // simple hashmap comprehension

    let squares_hashmap = c!{key => key*key, for key in 0..10};


    // hashmap comprehension from an Iterator
    // NOTE: we have to perform dereferencing: *key

    let map = c!{*key => key*key, for key in vec![1,2].iter()};


    // conditional hashmap comprehension

    let v: Vec<(&str, i32)> = vec![("one", 1), ("two", 2), ("three", 3)];
    let map = c!{key => val, for (key, val) in v, if val == 1 || val == 2};

    let mut expected: HashMap<&str, i32> = HashMap::new();
    expected.insert("one", 1);
    expected.insert("two", 2);

    assert_eq!(map, expected);


    // conditional hashmap comprehension from an Iterator
    // NOTE: we perform deferencing when using values

    let map = c!{*key => key*key, for key in vec![1,2].iter(), if *key % 2 == 0};
    let mut e: HashMap<i32, i32> = HashMap::new();
    e.insert(2, 4);

    assert_eq!(map, e);


    println!("Hello, world!");
}
