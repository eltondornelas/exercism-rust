fn main() {
    println!("1) hello = {}", hello());
    println!("2) reverse_string = {}", reverse_string("stressed"));
}

fn reverse_string(input: &str) -> String {
    let mut vec: Vec<char> = vec![];

    input.chars().rev().for_each(|c| {
        vec.push(c);
    });

    String::from_iter(&vec)
}

fn hello() -> &'static str {
    "Hello, world!"
}
