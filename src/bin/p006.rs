fn main() {
    let sum_of_squares: u32 = (1..101).map(|x: u32| x.pow(2)).sum();
    let square_of_sums = (1..101).sum::<u32>().pow(2);

    let res = square_of_sums - sum_of_squares;

    println!("Result is: {}", res);
}
