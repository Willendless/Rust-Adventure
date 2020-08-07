fn main() {
    let mut x = 3;
    println!("The value of x is: {}", x);
    x = 2;
    println!("The value of x is: {}", x);
    let x = 7;
    println!("The value of x is: {}", x);

    another_function(5);
}

fn another_function(x: i32) {
    println!("lalalla argument is: {}", x);

    let k = {
        let m = 5;
        m + 1
    };

    println!("kkkkk you are: {}", k);
    println!("six(): {}", six());

    for n in (1..5).rev() {
        println!("{}!", n);
    }
}

fn six() -> i32 {
    let c = if true { "I am six" } else { "I am not six" };
    println!("{}",c);
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 6 {
            break counter;
        }
    };
    result
}
