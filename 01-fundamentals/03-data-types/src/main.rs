fn main() {
    let server_port: u16 = 8080;
    let active_users: u32 = 1500;
    let cpu_usage: f32 = 72.5;
    let is_healthy: bool = true;
    let server_region: char = 'E';
    let total_users: u64 = active_users as u64;

    println!("Server Port: {}", server_port);
    println!("Active Users: {}", active_users);
    println!("CPU Usage: {}", cpu_usage);
    println!("Is Healthy: {}", is_healthy);
    println!("Server Region: {}", server_region);
    println!("Total Users: {}", total_users);

    let value: u16 = 300;
    let result: u8 = value as u8;

    println!("Value: {}", value);
    println!("Result: {}", result);

    let result2: u8 = u8::from(value);
    println!("Result2: {}", result2);
}
