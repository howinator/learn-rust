// Import (via `use`) the `fmt` module to make it available
use std::fmt;

// Define a structure for which `fmt::Display` will be implemented.
// This is a tuple struct named `Structure` that contains an `i32`
struct Structure(i32);

// To use the `{}` marker, the trait `fmt::Display` must be implemented manually for the type
impl fmt::Display for Structure {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// A structure holding two numbers. Debug will be derived.
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point
        write!(f, "({}, {})", self.0, self.1)
    }
}

// Define a structure where the fields are nameable for comparison.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

struct Complex {
    real: f64,
    imaginary: f64,
}

impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imaginary)
    }
}

fn main() {
    let struc = Structure(4);
    println!("This is the first structure {}", struc);


    let minmax = MinMax(0, 14);
    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small range is {small}", small = small_range, big = big_range);

    let point = Point2D {x: 3.3, y: 7.2};

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    let complex = Complex {real: 3.3, imaginary: 7.2};

    println!("Compare complex numbers");
    println!("Display: {}", complex);
    println!("Debug: {}", complex);

}
