// Make it work
// use std::mem::size_of_val;
// fn main() {
//     let c1 = 'a';
//     assert_eq!(size_of_val(&c1),1);
//
//     let c2 = '中';
//     assert_eq!(size_of_val(&c2),3);
//
//     println!("Success!");
// }

use std::mem::size_of_val;
fn main() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2: char = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!");
}
