/*
An algorithm, used to calculate the amount of prime numbers until a certain threshold
This version is optimized for performance by ChatGPT.
 */

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    let input = [6843, 1028, 3324, 1000000];

    for &n in &input {
        let result = generate_prime_list(n);
        println!("Found {} primes below {}", result.len(), n);
    }
    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}

fn is_prime(number: i32) -> bool {
    if number <= 1 {
        return false;
    }
    if number <= 3 {
        return true;
    }
    if number % 2 == 0 || number % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= number {
        if number % i == 0 || number % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn generate_prime_list(up_to: i32) -> Vec<i32> {
    let mut primes = vec![2];
    for number in (3..=up_to).step_by(2) {
        if is_prime(number) {
            primes.push(number);
        }
    }
    primes
}
