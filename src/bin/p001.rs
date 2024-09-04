fn main() {
    let numbers = 1..1000;
    let result: i32 = numbers.filter(|x| x % 3 == 0 || x % 5 == 0).sum();
    println!("{}", result);
}
