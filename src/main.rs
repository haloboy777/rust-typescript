// use std::fs;
// use std::fmt;

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

// struct Items(Vec<Item>);

// impl fmt::Display for Item {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             Item::Number(it) => write!(f, "{}", it),
//             Item::String(it) => write!(f, "{}", it),
//             Item::MyCustom(it) => write!(f, "{} <{}>", it.name, it.age),
//         }
//     }
// }

// impl fmt::Display for Items {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // for v in &self {
//         //     match v {
//         //         Item::Number(it) => write!(f, "{}", it),
//         //         Item::String(it) => write!(f, "{}", it),
//         //         Item::MyCustom(it) => write!(f, "{} <{}>", it.name, it.age),
//         //     }
//         // }
//         &self

//         Ok(())
//     }
// }

fn append(items: &mut Vec<Item>) {
    items.push(Item::String("Hello fem".into()));
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
    // println!("{}", foo);
}
