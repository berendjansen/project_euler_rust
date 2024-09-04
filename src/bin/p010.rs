use project_euler::primes::primes::find_primes_below;

fn main() {
    // Find sum of all primes below 2 million
    let primes = find_primes_below(2e6 as usize);
    let res: usize = primes.into_iter().sum();

    println!("Result is: {}", res)
}
