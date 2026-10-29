fn main() {

    let success_count: u32 = 850;
    let failed_count: u32 = 150;

    let total_request = calculate_total_requests(success_count, failed_count);
    let success_rate: f64 = calculate_success_rate(f64::from(success_count), f64::from(total_request));

    print_metrics(total_request, success_rate);
}

fn calculate_total_requests(success_count: u32, failed_count: u32) -> u32 {
    success_count + failed_count
}

fn calculate_success_rate(success_count: f64, total_count: f64) -> f64 {
    (success_count / total_count) * 100.0
}

fn print_metrics(total: u32, success_rate: f64) {
    println!("Total Requests: {}", total);
    println!("Success Rate: {}%", success_rate);
}