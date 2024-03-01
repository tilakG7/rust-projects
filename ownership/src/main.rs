fn main() {
    let mut my_str = String::from("Hello World");

    // example of slices
    // slice the Hello World string into 2 parts. Hello and world.
    let hello = &my_str[0..5];
    let hello2 = &my_str[..5];
    let world = &my_str[6..11];

    println!("Hello: {hello}");
    println!("World: {world}");
    println!("Notice how hello and hello2 have the same value: {hello}, {hello2}");

    my_str = String::from("A randome sentence");
    println!("Calling the first word function on string \"{my_str}\""); 
    println!("{}", first_word(&my_str));

}

fn first_word(my_str: &String) -> &str {
    let bytes = my_str.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &my_str[0..i];
        }
    }
    return &my_str[..];
}