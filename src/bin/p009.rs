fn main() {
    // There exists exactly one Pythagorean triplet for which a + b + c = 1000
    // Find the product abc.

    let f_a = |m: &usize, n: &usize| m.pow(2) - n.pow(2);
    let f_b = |m: &usize, n: &usize| 2 * m * n;
    let f_c = |m: &usize, n: &usize| m.pow(2) + n.pow(2);

    let (mut a, mut b, mut c) = (0usize, 0usize, 0usize);
    'outer: for m in 1usize..50 {
        for n in 1..m {
            a = f_a(&m, &n);
            b = f_b(&m, &n);
            c = f_c(&m, &n);

            if (a + b + c) == 1000 {
                break 'outer;
            }
        }
    }
    let res = a * b * c;
    println!("A: {}, B: {}, C: {}", a, b, c);
    println!("Result is: {}", res);
}
