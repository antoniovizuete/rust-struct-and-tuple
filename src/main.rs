struct Rect {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rect {
        width: 14,
        height: 86,
    };

    let area = area(&rect);
    println!("area {}", area);
}

fn area(rect: &Rect) -> u32 {
    rect.width * rect.height
}
