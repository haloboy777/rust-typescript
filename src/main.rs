// use std::fs;
use std::fmt;

// enum Colors {
//     Red,
//     Blue,
//     Green,
//     Yellow,
// }

// impl Colors {
//     fn is_green(&self) -> bool {
//         if let Colors::Green = self {
//             return true;
//         } else {
//             return false;
//         }
//     }
//     fn is_green_parts(&self) -> bool {
//         match self {
//             Colors::Red => return false,
//             Colors::Blue => return true,
//             Colors::Green => return false,
//             Colors::Yellow => return true,
//         }
//     }
// }

// fn print_color(color: Colors) -> () {
//     match color {
//         Colors::Blue => println!("blue"),
//         Colors::Red => println!("red"),
//         Colors::Green => println!("green"),
//         Colors::Yellow => println!("yellow"),
//     }
// }

struct Custom {
    age: usize,
    name: String,
}

enum Item {
    Number(usize),
    String(String),
    MyCustom(Custom),
}

struct DisplayItem(Vec<Item>);

impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = match self {
            Item::Number(it) => write!(f, "{}", it),
            Item::String(it) => write!(f, "{}", it),
            Item::MyCustom(it) => write!(f, "{} <{}>", it.name, it.age),
        };
        return Ok(());
    }
}

impl fmt::Display for DisplayItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        /*
         we need to get vector from DisplayItem wrapper
        we added that wrapper as we can not implement
        a trait for a type defined outside the current crate
        ---
        OFFICIAL ERROR: only traits defined in the current crate can be
        implemented for types defined outside of the crate
        */
        let DisplayItem(vector) = &self;

        for vector_item in vector {
            // why did this work?!??!?!??!

            // this worked because writeln! and write! gives a Result
            let _ = match vector_item {
                Item::MyCustom(it) => writeln!(f, "{} ({})", it.name, it.age),
                Item::Number(it) => writeln!(f, "{}", it),
                Item::String(it) => writeln!(f, "{}", it),
            };
        }

        return Ok(());
    }
}

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello fem".into()));
}

fn append_item(items: &mut Vec<Item>) {
    items.push(Item::MyCustom(Custom {
        age: 10,
        name: "Ayush".into(),
    }));
}
fn append_number(items: &mut Vec<Item>) {
    items.push(Item::Number(10));
}
fn main() {
    // --------------------------------------------------------------
    // let data = vec![1, 2, 3];
    // let mut foo = data.iter().map(|x| x + 1);
    // let mut new_vec = vec![];
    // while let Some(x) = foo.next() {
    //     new_vec.push(x);
    // }
    // println!("{:?}", new_vec);

    // --------------------------------------------------------------
    // let contents = fs::read_to_string("lines").unwrap();

    // contents
    //     .lines()
    //     .enumerate()
    //     .filter(|(idx, _)| *idx % 2 == 0)
    //     .skip(2)
    //     .take(2)
    //     .for_each(|(_, val)| println!("{}", val));
    // --------------------------------------------------------------

    // print_color(Colors::Blue);
    // print_color(Colors::Green);
    // print_color(Colors::Red);
    // print_color(Colors::Yellow);

    // --------------------------------------------------------------
    // let foo = Colors::Green;

    // println!("{}", foo.is_green());
    // println!("{}", foo.is_green_parts());
    // --------------------------------------------------------------
    let mut foo: Vec<Item> = vec![];

    append(&mut foo);
    append_item(&mut foo);
    append_number(&mut foo);
    println!("{}", DisplayItem(foo));
}
