fn main() {
    println!("{}", fibonacci(25));
}

fn fibonacci(n: i32) -> i32 {

    if n == 2 || n == 3 {
        return n;
    }

    let mut a = 1;
    let mut b = 1;

    for i in 3..n+1 {
        if i % 2 == 0 {
            b = a + b
        } else {
            a = a + b
        }
    }

    return if a > b { a } else { b };
}
