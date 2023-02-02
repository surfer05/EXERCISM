fn main() {
    println!("Hello,World!");
}

pub fn reverse(input: &str) -> String {
    let mut s = String::new();
    for element in input.chars().new() {
        s.push(element);
    }
    return s;
}
