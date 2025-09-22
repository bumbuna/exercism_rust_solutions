pub fn is_armstrong_number(num: u32) -> bool {
    let num_len = num.to_string().len() as u32;
    let mut num2 = num;
    let mut arm_sum = 0;
    while num2 > 0 {
        let x = num2 % 10;
        num2 /= 10;
        arm_sum += x.pow(num_len);
    }
    arm_sum == num
}
