fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &y;     // z is an immutable reference to y

    *y += 1; // Modifying x through y is allowed
    println!("x = {}", x); // Output: x = 6

    // *z += 1; // This will cause a compile-time error because z is immutable
}