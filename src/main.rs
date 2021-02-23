fn main() {
    let rect: (u32, u32) = (14, 86);

    let area = area(rect);
    println!("area {}", area);
}

fn area(rect: (u32, u32)) -> u32 {
    rect.0 * rect.1
}
