pub fn num_to_string(num: u32) -> String {
    match num {
        0 => String::from("zero"),
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        _ => String::from("other"),
    }
}

pub fn unwrap_or_default(x: Option<u32>, v: u32) -> u32 {
    match x {
        Some(val) => val,
        None => v,
    }
}
