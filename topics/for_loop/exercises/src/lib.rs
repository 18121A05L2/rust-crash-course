pub fn sum(nums: Vec<i32>) -> i32 {
    let mut total = 0;
    for num in nums.iter() {
        total += num;
    }
    total
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    vec![i; n]
}
