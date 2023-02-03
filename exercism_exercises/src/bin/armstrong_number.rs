pub fn is_armstrong_number(num: u32) -> bool {
    let length = num.to_string().len() as u32;

    let mut n:u64 = num as u64;
    let mut sum:u64 = 0;

    while n>0 {
        sum += (n%10).pow(length);
        n/=10;
    }
    
    sum ==num as u64
}
