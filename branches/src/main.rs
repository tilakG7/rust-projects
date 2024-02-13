

fn main() {
    let x : u32 = 5;
    if x >= 6u32 {
        println!("In IF!");
    }
    else if x == 5u32 {
        println!("In ELSE IF!");
    }
    else {
        println!("In ELSE")
    }

    let y = if x == 5 { let x = 4; x + 4} else {5};
    println!("y = {y}");

    let mut i = 5;

    while i != 10 {
        println!("i: {i}");
        i += 1;
    }

    let mut counter = 0;
    let i = loop {
        counter += 1;
        if counter == 100 {
            break counter;
        }
    };
    println!("i: {i}");

    let mut count = 0;
    'counting_up: loop {
        println!("Count = {count}");
        let mut remaining = 10;

        loop {
            println!("Reamaining = {remaining}");
            if remaining == 7 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let my_array = [4u32; 10];
    let my_tup  = (1u32, 1u32);

    let mut i = 0;
    while i < 10 {
        println!("{}", my_array[i]);
        i += 1;
    }

    for element in my_array {
        println!("Element: {element}");
    }

    for countdown_timer in (1..10).rev() {
        println!("{countdown_timer}");
    }
    println!("We have liftoff!");

}
