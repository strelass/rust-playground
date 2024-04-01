// fn main() {
//     let s = sum(1 , 2);
//     assert_eq!(s, 3);
//
//     println!("Success!");
// }
//
// fn sum(x: i32, y: i32) -> i32 {
//     x + y;
// }

fn main() {
    let s: i32 = sum(1 , 2);
    let s1: i32 = sum_2(1 , 2);
    assert_eq!(s, 3);
    assert_eq!(s1, 3);

    println!("Success!");
}

fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn sum_2(x: i32, y: i32) -> i32 {
    x + y
}
