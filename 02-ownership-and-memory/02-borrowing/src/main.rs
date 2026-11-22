fn main() {
    let mut payload = String::from("CreateUser");

    print_payload(&payload);

    let is_valid = validate_payload(&payload);
    println!("Is Valid: {is_valid}");

    if is_valid {
        append_status(&mut payload);
    }

    print_payload(&payload);
}

fn print_payload(payload: &str) {
    println!("Payload: {payload}");
}

fn validate_payload(payload: &str) -> bool {
    !payload.is_empty()
}

fn append_status(payload: &mut String) {
    payload.push_str(":Validated");
}