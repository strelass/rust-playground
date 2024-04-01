// Make it work, don't modify `implicitly_ret_unit` !
// fn main() {
//     let _v: () = ();
//
//     let v = (2, 3);
//     assert_eq!(v, implicitly_ret_unit());
//
//     println!("Success!");
// }
//
// fn implicitly_ret_unit() {
//     println!("I will return a ()");
// }
//
// // Don't use this one
// fn explicitly_ret_unit() -> () {
//     println!("I will return a ()");
// }

fn main() {
    let x: () = ();

    let _v: (i32, i32) = (2, 3);
    assert_eq!(x, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() {  // by default returns ()
    println!("I will return a ()");
}

// Don't use this one
fn explicitly_ret_unit() -> () {  // raises warning
    println!("I will return a ()");
}
