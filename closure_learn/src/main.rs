// use std::thread;
fn main() {
    let list = vec![1, 2, 3];
    let list2: Vec<i32> = list.iter().filter(|x| *x % 2 == 1).map(|x| x + 1).collect();
    println!("{:?}", list2);
}
