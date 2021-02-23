struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rect {
        width: 14,
        height: 86,
    };

    let area = rect.area();
    println!("area {}", area);
}
