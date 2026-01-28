#![allow(clippy::clone_on_copy)]

fn generate_json(count: usize) -> String {
    // Inefficiency #1: string concatenation without with_capacity
    let mut json = String::new();
    json.push('[');
    for i in 0..count {
        if i > 0 {
            json.push(',');
        }
        json.push_str(&i.to_string());
    }
    json.push(']');
    json
}

fn process_json(data: &str) -> Result<Vec<u32>, serde_json::Error> {
    // Inefficiency #2: re-parses every call (no caching)
    let parsed: Vec<u32> = serde_json::from_str(data)?;
    // Inefficiency #3: no capacity hint on new vec
    // Inefficiency #4: redundant copy instead of returning parsed directly
    let mut result = Vec::new();
    for &item in &parsed {
        result.push(item);
    }
    Ok(result)
}

fn sum_numbers(numbers: &[u32]) -> u64 {
    // Inefficiency #5: unnecessary allocation via to_vec on a slice
    let owned = numbers.to_vec();
    // Inefficiency #6: pointless clone of Copy type in manual loop
    let mut total: u64 = 0;
    for item in &owned {
        let val = item.clone();
        total += val as u64;
    }
    total
}

fn main() {
    let json_data = generate_json(10_000);

    for i in 0..1000 {
        // Inefficiency #7: clones the entire JSON string each iteration
        let data = json_data.clone();
        let numbers = process_json(&data).unwrap();
        // Inefficiency #8: clones the Vec before passing to sum_numbers
        let numbers_clone = numbers.clone();
        let sum = sum_numbers(&numbers_clone);

        if i == 0 {
            println!("Sum: {}", sum);
        }
    }

    println!("Completed 1000 iterations.");
}
