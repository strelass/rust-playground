// Remove something to make it work
// fn main() {
//     let x: i32 = 5;
//     let mut y: u32 = 5;
//
//     y = x;
//
//     let z = 10; // Type of z ?
//
//     println!("Success!");
// }

fn main() {
    let x: i32 = 5;
    let mut y = 5;  // removed type annotation to use default i32 type

    y = x;

    let z: i32 = 10;

    println!("Success!");
}
