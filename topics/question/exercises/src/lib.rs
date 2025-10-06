fn parse(s: &str) -> Result<u32, String> {
    match s.parse() {
        Ok(val) => Ok(val),
        Err(_) => Err("Failed to parse string into u32".to_string()),
    }
}

pub fn sum(nums: &[&str]) -> Result<u32, String> {
    println!(" nums :: {:?}", nums);
    let mut total = 0;
    for i in nums {
        total += parse(i)?;
    }
    println!("{total}");
    Ok(total)
}
