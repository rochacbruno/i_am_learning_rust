#![allow(dead_code)]
fn main() {
    println!("--- Structs ---");

    // Structs (structures)
    // There are 3 types of structs in Rust

    // Unit like structs (useful for generics, atomic symbols, etc...)
    struct Nothing;

    // Tuple structs
    struct TupleRGBColor(u8, u8, u8);
    let white = TupleRGBColor(255, 255, 255);
    // Items accessed by index position
    println!("Red: {}, Green: {}, Blue: {}", white.0, white.1, white.2);

    // Data structs
    struct DataRGBColor {red: u8, green: u8, blue: u8};
    let white = DataRGBColor {red: 255, green: 255, blue: 255};
    // items accessed by field name
    println!("Red: {}, Green: {}, Blue: {}", white.red, white.green, white.blue);

    // Structs can be used as struct fields
    #[derive(Debug)]
    struct Point(f32, f32);

    #[derive(Debug)]
    struct Rectangle { p1: Point, p2: Point };

    let rect = Rectangle {p1: Point(0.3, 0.4), p2: Point(0.1, 0.3)};

    println!("{:#?}", rect);

    println!("--- Match on a struct ---");
    // Structs can be used in a match
    let point = Point(0.2, 0.5);
    match point {
        Point(x, _) if x > 0.0 => println!("x is greater than 0"),
        Point(_, y) if y > 0.0 => println!("y is greater than 0"),
        _ => println!("x and y are 0")
    }

    // Structs can have method implementation
    struct Area {
        width: u32,
        height: u32
    }

    println!("--- implementations (a.k.a methods) ---");
    impl Area {
        fn is_square(&self) -> bool {
            self.width == self.height
        }
    }   
    
    let room = Area {width: 30, height: 30};
    println!("The room is squared: {}", room.is_square());

    // You can have multiple `impl` blocks, on demand
    impl Area {
        fn total(&self) -> u32 {
            self.width * self.height
        }
    }

    println!("The room total area is: {}", room.total());

    println!("--- Mutable ---");
    // To change value of fields the variable must be `mutable`
    let mut room = Area {width: 30, height: 30};
    println!("The room is squared: {}", room.is_square());

    room.height = 50;
    println!("The room is still a square?: {}", room.is_square())

}
