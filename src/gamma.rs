use crate::factorial::factorial;

fn ngamma(n: u64) -> f64 {
    if n == 0 {
        return 1.0;
    } else {
        return factorial(n - 1) as f64;
    }
}

pub fn gamma(x: f64) -> f64 {
    // Check if x can be represented as an integer
    if x.fract() == 0.0 {
        return ngamma(x as u64);
    } else {
        return x;
    }
}