fn main() {
    //Small example in rust
    let mut x: i32 = 6;
    print!("{x}");
    while x != 1 {
        if x % 2 == 0 {
            x = x / 2;
        } else {
            x = 3 * x + 1;
        }
        // Does this loop always end?
        print!("{x}")
    }

    println!() // Line space
}
