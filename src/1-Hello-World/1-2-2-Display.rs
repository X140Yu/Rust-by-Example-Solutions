use std::fmt; // Import `fmt`

struct Complex {
    real: f64,
    imag: f64,
}

// Implement `Debug` for `Complex`.
impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Complex {{ real: {}, imag: {} }}", self.real, self.imag)
    }
}

// Implement `Display` for `Complex`.
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} + {}i", self.real, self.imag)
    }
}

fn main() {

    let complex = Complex{ real: 3.3, imag: 7.2 };
    println!("Display: {}", complex);
    println!("Debug: {:?}", complex);

}