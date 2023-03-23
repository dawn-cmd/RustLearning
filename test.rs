fn main() {
    let mut x = 0.0;
    let mut y: f64 = 0.0;
    for _i in 0..5 {
        let yd: f64 = 2.0 * x - 3.0 * x * y;
        y = yd * 0.1 + y;
        x = x + 0.1;
    }
    println!("{y}");
}
