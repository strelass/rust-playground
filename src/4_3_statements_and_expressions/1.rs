// Make it work with two ways
// fn main() {
//     let v = {
//         let mut x = 1;
//         x += 2
//     };
//
//     assert_eq!(v, 3);
//
//     println!("Success!");
// }

fn main() {
    let v: i32 = {
        let x: i32 = 1;
        x + 2
    };

    assert_eq!(v, 3);

    let v: i32 = {
        let mut x: i32 = 1;
        x += 2;
        x
    };

    assert_eq!(v, 3);

    let v: () = {
        let mut _x: i32 = 1;
        _x += 2
    };

    assert_eq!(v, ());

    println!("Success!");
}
