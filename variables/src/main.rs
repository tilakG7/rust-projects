
fn main() {
    let spaces = "   ";
    let spaces : usize = spaces.len();

    let guess = 1_000_000u32;
    println!("Num spaces: {spaces}");
    println!("Guess: {guess}");

    let x = 2.0; // f64
    let y: f32 = 3.0;
    println!("x: {x}");
    println!("y: {y}");

    let xy = 3/2;
    println!("3/2 = {xy}");

    let my_tup : (i32, f64, u8)  = (3_000, 5445.3, 244);
    let (x, y, z) = my_tup;
    println!("x: {x}\n y: {y}\n z: {z}");
    let x = my_tup.2;
    let y = my_tup.1;
    let z = my_tup.0;
    println!("x: {x}\n y: {y}\n z: {z}");

    let _a : [u32; 6] = [1, 2, 3, 4, 5, 6];
    let _b = [4f32; 5];

    another_function();
    another_function();

}

fn another_function() {
    println!("Calling another function!");
}