fn main() {
    let mut previous = 0;
    let mut current = 1;
    let mut fib = previous + current;
    let mut res = 0;

    while fib < 4000000 {
        previous = current;
        current = fib;
        fib = previous + current;

        if fib % 2 == 0 {
            res += fib;
        }
    }
    println!("Result: {}", res);
}