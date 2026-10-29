
fn main() {

    let max_retries: u32 = 5;
    println!("Simulated with loop");
    simulate_with_loop(max_retries);
    println!("Simulated with while");
    simulate_with_while(max_retries);
}

fn simulate_with_loop(max_retries: u32) {
    let mut attempt: u32 = 1;

    loop {
        if attempt > max_retries {
            println!("Maximum retry limit reached");
            break;
        }

        let request_succeeded = simulate_request(attempt);

        if request_succeeded {
            println!("Attempt {attempt}: Request succeeded");
            break;
        } else {
            println!("Attempt {attempt}: Request failed");
            attempt += 1;
        }
    }
}

fn simulate_with_while(max_retries: u32) {
    let mut attempt: u32 = 1;
    let mut request_succeeded = false;

    while !request_succeeded && attempt <= max_retries {
        request_succeeded = simulate_request(attempt);

        if request_succeeded {
            println!("Attempt {attempt}: Request succeeded");
            break;
        } else {
            println!("Attempt {attempt}: Request failed");
            attempt += 1;
        }
    }

    if !request_succeeded {
        println!("Maximum retry limit reached");
    }
}
fn simulate_request(attempt: u32) -> bool {
    attempt > 2
}