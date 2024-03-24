use crate::expressions::TronType;

pub fn clock_impl(_args: &Vec<TronType>) -> TronType {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time")
        .as_millis();
    println!("{}", now);
    TronType::Number(now as f32 / 1000.0)
}
