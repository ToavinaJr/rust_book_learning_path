pub const Y : u32 = 23;
pub fn test () {
    let x = 25;
    let x = x + 1;
    const Y : u32 = 23;

    println!("let x = {}", x);
    println!("Const y = {}", Y);

    let guess : u32 = "42".parse().expect("Not a number!");
    println!("Guess = {}", guess);

    // u(n) in [0; 2^n - 1] 
    let x : u8 = 255;

    println!("x = {}", x);
}