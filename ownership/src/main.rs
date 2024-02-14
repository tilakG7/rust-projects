

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");

    println!("{}", s);

    let s1 = String::from("Helloooooo");
    let s2 = s1.clone();

    println!("s1: {s1}");
    println!("s2: {s2}");
}
