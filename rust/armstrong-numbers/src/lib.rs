pub fn is_armstrong_number(num: u32) -> bool {
    let mut temp_num = num;
    let mut nums: Vec<u32> = Vec::new();
    while temp_num >= 10 {
        nums.push(temp_num % 10);
        temp_num /= 10;
    }
    nums.push(temp_num);
    nums.iter()
        .map(|&x| x.pow(nums.len() as u32))
        .collect::<Vec<u32>>()
        .iter()
        .sum::<u32>()
        == num
}
