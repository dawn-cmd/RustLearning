#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}
impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rect) -> bool {
        std::cmp::min(other.width, other.height) < std::cmp::min(self.width, self.height)
            && std::cmp::max(other.width, other.height) < std::cmp::max(self.width, self.height)
    }
    fn square(size: u32) -> Rect {
        Rect { width: (size), height: (size) }
    }
}
fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };
    let rect2 = Rect::square(40);
    let rect3 = Rect {
        width: 40,
        height: 20,
    };
    println!("{}, {}, {}", rect1.area(), rect2.area(), rect3.area());
    println!("{}", rect1.can_hold(&rect2));
    println!("{}", rect1.can_hold(&rect3));
}
