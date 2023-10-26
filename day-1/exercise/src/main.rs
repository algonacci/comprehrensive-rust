fn main() {
    let mut x: i8 = 6;
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        print!(" -> {x}")
    }
    println!();
    fizz_buzz(30);

    let s1: &str = "World";
    let mut s2: String = String::from("Hello ");
    s2.push_str(s1);
    println!("{s2}");
    println!("===========");
    let mut rect = Rectangle {
        width: 5,
        height: 10,
    };
    println!("Old area: {}", rect.area());
    rect.inc_width(5);
    println!("New area: {}", rect.area());

    println!("===========");
    println!("Coin toss: {}", pick_one("head", "tail"));
    println!("Cash prize: {}", pick_one(500, 100));
}

fn fizz_buzz(n: u32) {
    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            println!("{i} -> Fizz Buzz");
        } else if i % 3 == 0 {
            println!("{i} -> Fizz");
        } else if i % 5 == 0 {
            println!("{i} -> Buzz");
        } else {
            println!("{i}");
        }
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn inc_width(&mut self, delta: u32) {
        self.width += delta;
    }
}

fn pick_one<T>(a: T, b: T) -> T {
    if std::process::id() % 2 == 0 {
        a
    } else {
        b
    }
}
