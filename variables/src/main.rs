fn main() {
    let x = 10;
    println!("{}", fibo(x));
}
fn fibo(x: i32) -> i32{
    if x == 0 || x == 1 {
        x
    } else {
        fibo(x - 1) + fibo(x - 2)
    }
}
