fn main() {
    let mut s = String::from("Hello World!");
    let len = getSize(&s);
    println!("The length of {} is {}", s, len);
}

fn getSize(s: & String) -> usize {
    // s.push_str(" My name is Tilak");
    s.len()
}