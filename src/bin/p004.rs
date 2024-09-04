fn is_palindrome(n: usize) -> bool {
    let mut tmp = n;
    let mut rev = 0;
    while tmp != 0 {
        rev = rev * 10 + tmp % 10;
        tmp = tmp / 10;
    }
    rev == n
}

fn main() {
    let mut res = 0;
    let mut p = 990;
    while p > 100 {
        for q in (100..1000).rev() {
            let tmp = p * q;
            if tmp > res && is_palindrome(tmp) {
                res = tmp;
                break;
            } else if tmp < res {
                break;
            }
        }
        p -= 11;
    }
    println!("Result: {}", res);
}
