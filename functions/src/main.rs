const TILAK : &str = "Tilak";
fn foo(mut var : u32) {
    println!("Var: {var}");
    var = 3;
    println!("Var: {var}");
}

fn bar() -> i32 {
    4
}

fn main() {
    println!("Tilak: {TILAK}");
    println!("Hello, world!");
    let i = 3u32;
    foo(i);

    let xx = {
        let y = 3;
        y*3 // adding semicolon to the end of an expression, turns it into a statement
    };

    println!("x = {xx}");

    let y = bar();
    println!("y: {y}");
}
