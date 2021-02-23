fn main() {
    let width = 14;
    let height = 86;

    let area = area(width, height);
    println!("area {}", area);
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
