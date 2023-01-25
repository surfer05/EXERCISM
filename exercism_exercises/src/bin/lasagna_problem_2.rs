fn main(){
    println!("hello world!");
}
pub fn expected_minutes_in_oven()->i32{
    40
}
pub fn remaining_minutes_in_oven(actual_minutes:i32)->i32{
    40 - actual_minutes
}
pub fn preparation_time_in_minutes(number_of_layers:i32)->i32{
    number_of_layers*2
}
pub fn elapsed_time_in_minutes(number_of_layers:i32 , number_of_minutes:i32)->i32{
    number_of_minutes + (number_of_layers*2)
}