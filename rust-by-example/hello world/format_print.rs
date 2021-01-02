fn main() {
    println!("{} days", 31i64);

    // Positional arguments
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb} {object}",
             object = "the laze dog",
             subject = "the quick brown fox",
             verb = "jumps over");

    // Special formatting
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // right-align text with a specified width.
    println!("{number:>width$}", number=1, width=6);

    // pad numbers with 0
    println!("{number:>0width$}", number=1, width=6);

    println!("My name is {0}, {1} {0}", "Bond", "James");    

    #[allow(dead_code)]
    struct Structure(i32);

    // println!("This struct `{}` won't print...", Structure(3));

    let pi = 3.1415926;

    println!("pi is roughly {:.3}", pi);
}
