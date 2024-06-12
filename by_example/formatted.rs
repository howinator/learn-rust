fn main() {
    // In general, the `{}` will be automatically replaced with any arguments. These will be
    // stringified
    println!("{} days", 31);

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start at 0 immediately
    // after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // Named arguments
    println!("{subject} {verb}, {object}", object="the lazy dog", subject="the quick brown fox", verb="jumbs over");

    // Different formatting can be invoked by specifying the format character
    println!("Base 10:                 {}", 69420);
    println!("Base 2 (binary):         {:b}", 69420);
    println!("Base 8 (octal):          {:o}", 69420);
    println!("Base 16 (hex):           {:x}", 69420);

    // Right justify text with a specified width
    println!("{number:>5}", number=1);
    // pad numbers with zeroes
    println!("{number:0>5}", number=1);
    //left-adjust by flipping the sign
    println!("{number:0<5}", number=1);

    //use named arguments in the format specifier by appending a `$`
    println!("{number:0>width$}", number=1, width=5);

    // Rust will check for correct number of args
    println!("My name is {0}, {1} {0}", "Bond", "James");

    #[allow(dead_code)]
    struct Structure(i32);

    // println!("This struct {} wont print", Structure(3));

    // For Rust 1.58 and above, you can directly capture the argument from a surrounding variable.
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

}
