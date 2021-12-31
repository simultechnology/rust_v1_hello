use std::fmt::Formatter;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: i32,
    children: i32,
    favorite_color: Color,
}

impl Person {
    pub fn print(self)->String {
        format!("name = {}, age = {}, children = {}, favorite_color = {}",
                self.name, self.age, self.children, self.favorite_color
        )
    }
}

#[derive(Debug)]
pub enum Color {
    Red(String),
    Green,
    Blue,
}

impl std::fmt::Display for Color {
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        write!(fmt, "Color!!")
    }
}

fn main() {
    let p = Person {
        name: "tatu".to_string(),
        age: 44,
        children: 2,
        favorite_color: Color::Green,
    };

    let color = Color::Red("Hello".to_string());

    match color {
        Color::Red(s) => println!("It's red {}", s),
        Color::Green => println!("It's green"),
        Color::Blue => println!("It's blue"),
    };
    let color2 = Color::Blue;
    println!("Color : {:?}", color2);

    // println!("Hello, algorithms, from {}", p.print());
    println!("Hello, algorithms, from {:?}", p);
}
