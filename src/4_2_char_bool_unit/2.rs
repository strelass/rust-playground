// Make it work
// fn main() {
//     let c1 = "中";
//     print_char(c1);
// }
//
// fn print_char(c : char) {
//     println!("{}", c);
// }

fn main() {
    let c1: char = '中'; // single quote for chars double for strings
    print_char(c1);
}

fn print_char(c: char) {
    println!("{}", c);
}
