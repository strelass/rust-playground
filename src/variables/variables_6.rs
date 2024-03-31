// Remove a line in the code to make it compile
// fn main() {
//     let mut x: i32 = 1;
//     x = 7;
//     // Shadowing and re-binding
//     let x = x;
//     x += 3;
//
//
//     let y = 4;
//     // Shadowing
//     let y = "I can also be bound to text!";
//
//     println!("Success!");
// }

fn main() {
    let mut x: i32 = 1;
    x = 7;
    // Shadowing and re-binding
    // let mut x: i32 = x;
    // should be mutable or removed as the task suggests
    x += 3;


    let y: i32 = 4;
    // Shadowing
    let y: &str = "I can also be bound to text!";

    println!("Success!");
}
