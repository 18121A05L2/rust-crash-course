pub fn parse_and_add(a: &str, b: &str) -> u32 {
    let a1: u32 = a.parse().expect("Failed to parse variable");
    let b1: u32 = b.parse().expect("Failed to parse variable");
    a1 + b1
}

pub fn unwrap_and_add(x: Option<u32>, y: Option<u32>) -> u32 {
    let x1 = x.unwrap();
    let x2 = y.unwrap();
    x1 + x2
}
