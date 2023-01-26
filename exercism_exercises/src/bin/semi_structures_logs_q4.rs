fn main(){
    println!("Hello, World!");
}

pub enum LogLevel {
    Info,
    Warning,
    Error,
}
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
}
    
}
pub fn info(message: &str) -> String {
    return "[INFO]: ".to_string() +message
}
pub fn warn(message: &str) -> String {
    return "[WARNING]: ".to_string() +message
}
pub fn error(message: &str) -> String {
    return "[ERROR]: ".to_string()+message
}