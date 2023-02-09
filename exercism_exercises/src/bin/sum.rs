pub fn square_of_sum(n: u32) -> u32 {
        let mut m = n;
    let mut sum:u32 = 0;
    while m>0{
        sum += m;
        m-=1;
    } 
    sum*sum
}
pub fn sum_of_squares(n: u32) -> u32 {
    let mut m = n;
    let mut sum:u32 =0;
    while m>0{
        sum+= m*m;
        m-=1;
    }
    sum
}
pub fn difference(n: u32) -> u32 {
    square_of_sum(n)-sum_of_squares(n)
}