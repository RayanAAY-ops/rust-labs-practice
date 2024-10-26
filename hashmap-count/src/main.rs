use rand::Rng;
use std::collections::HashMap;
use std::time::Instant;

// A higher-order function to measure execution time
fn measure_time<F, R>(func: F) -> (R, std::time::Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = func(); // Call the passed function
    let duration = start.elapsed(); // Measure elapsed time
    (result, duration) // Return the result and the duration
}


fn logic(numbers: Vec<i32>) -> Vec<(i32, u32)> {
    let mut frequencies = HashMap::new();
    for num in numbers {
        let frequency = frequencies.entry(num).or_insert(0);
        *frequency = *frequency + 1;
    }

    let mut result: Vec<(i32, u32)> = Vec::new();

    for element in frequencies {
        result.push((element.0, element.1));
    }
    // result.sort_by_key(|k| k.0); // Sorting the vector by key
    return result;
}

fn generate_numbers(range: i32) -> Vec<i32> {
    let mut placeholder: Vec<i32> = Vec::with_capacity(range as usize); // Convert to usize only here

    // Create a random number generator
    let mut rng = rand::thread_rng();

    for _ in 0..range {
        placeholder.push(rng.gen_range(0..range)); // No need to change types, keep it as i32
    }
    placeholder // Return the vector directly
}
fn main() {
    let upper_limit: i32 = 1_000_000; // Adjust this value as needed

    // Measure the time for number generation
    let (numbers, duration_gen) = measure_time(|| generate_numbers(upper_limit));

    // Measure the time for the logic function
    let (results, duration_logic) = measure_time(|| logic(numbers));

    // Get a slice of the first 100 results
    let results_slice = &results[..results.len().min(100)];

    // Print the timing results
    println!(
        "Time taken to generate numbers: {:?}",
        duration_gen
    );
    println!(
        "Time taken to calculate frequencies: {:?}",
        duration_logic
    );

    // Print the frequency hashmap of the first elements
    /*println!(
        "The frequency hashmap of the first elements: {:?}",
        results_slice
    );
    */
}