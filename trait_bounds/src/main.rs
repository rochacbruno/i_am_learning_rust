use std::fmt;


struct Dwarf {
    name: String
}

struct Elf {
    name: String
}

struct HalfOrc {
    name: String
}

struct Human {
    name: String
}

struct HalfElf {
    name: String
}


// impl

pub trait Constitution {
    fn constitution_bonus(&self) -> u8 {
        0
    }
}

impl Constitution for Dwarf {
    fn constitution_bonus(&self) -> u8 {
        2
    }
}

impl Constitution for HalfOrc {
    fn constitution_bonus(&self) -> u8 {
        1
    }
}

impl Constitution for Human {}

impl Constitution for Elf {}


pub trait Elvish {}
impl Elvish for Elf {}
impl Elvish for HalfElf {}

impl fmt::Display for Elf {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}


// bounds

// pub fn speak_elvish<T: Elvish>(character: T) where T: fmt::Display {
pub fn speak_elvish<T: Elvish + fmt::Display>(character: T) {
    println!("{} says {}", character, String::from("yes"));
}



fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}


// default behavior

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

// data

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impls

impl Summary for NewsArticle {

    fn summarize_author(&self) -> String {
        format!(">>>{}", self.author)
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}


pub fn notify_ref<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}



pub fn notify<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}



// Conditional IMPLS

// Create a struct of T
struct Pair<T> {
    x: T,
    y: T,
}

// Implements a ::new() for the pair struct having any type of T (generic)
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {
            x,
            y,
        }
    }
}


struct Dummy {
    name: String
}

// Conditionally implements cmp_display for Pair holding T
// where T must implements PartialOrd and Display traits
// ex: does not works for when T is Dummy but works for String and i*

impl<T: fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}


fn main() {

    //D & D example
    let my_dwarf = Dwarf{name: String::from("NellDwaarf")};
    println!("{}", my_dwarf.constitution_bonus());

    let my_horc = HalfOrc{name: String::from("NellHorc")};
    println!("{}", my_horc.constitution_bonus());

    let my_elf = Elf{name: String::from("Elf")};
    println!("{}", my_elf.constitution_bonus());

    let human = Human{name: String::from("Human")};
    println!("{}", human.constitution_bonus());

    speak_elvish(my_elf);


    // CHARS example
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);


    // SUMARRY
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());


    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };
    
    notify_ref(&article);  // passing reference no move

    println!("New article available! {}", article.summarize());

    notify(article);  // passing value, move!!


    let pair = Pair::new(2, 6);
    pair.cmp_display();

    // This will fail because Dummy does not implements PartialOrd and Display
    // to satisfy the conditional impl of cmp_display
    // let pair2 = Pair::new(Dummy{name: 'a'}, Dummy{name: 'b'});
    // pair2.cmp_display();

}
