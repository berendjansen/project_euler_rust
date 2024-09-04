use project_euler::primes::prime_factorisation::prime_factorisation;
use std::collections::HashMap;

fn main() {
    // What is the smallest positive number that is
    // evenly divisible by all the numbers from 1 to 20?

    let factorisations = (2..21).map(|x| prime_factorisation(x)).collect::<Vec<_>>();

    let mut max_power_map: HashMap<u32, u32> = HashMap::new();

    for factorisation in factorisations {
        for f in factorisation.factors {
            let current_max = max_power_map.entry(f.value).or_insert(0);
            if f.power > *current_max {
                *current_max = f.power
            }
        }
    }

    let res: u32 = max_power_map.iter().map(|(v, p)| v.pow(*p)).product();

    println!("Result is: {:#?}", res);
}
