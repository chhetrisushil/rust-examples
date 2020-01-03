mod custom_lib;

fn main() {
    let arr: [i32; 6] = [10, 23, 30, 40, 42, 50];
    let len: i32 = arr.len() as i32;
    let start: i32 = 0;
    let end: i32 = len - 1;
    let number_to_find: i32 = 42;
    println!("number found at: {}", custom_lib::bst::run(number_to_find, &arr, start, end));
}
