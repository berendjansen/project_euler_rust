fn main() {
    let num: usize = 600851475143;
    // let num: usize = 13195;
    
    let mut res = num;
    let mut div = 2;

    while res > 1 {
        if res % div == 0 {
            res = res / div;
        } else {
            div += 1;
        }
    }
    println!("{}", div);
}