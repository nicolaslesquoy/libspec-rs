/// Helper functions

pub fn sieve(start: u64, end: u64) -> Vec<u64> {
    let mut sieve_array = vec![true; (end + 1) as usize];
    sieve_array[0] = false;
    sieve_array[1] = false;
    for i in (4..end + 1).step_by(2) {
        sieve_array[i as usize] = false;
    }
    let mut i = 3;
    while i * i <= end + 1 {
        if sieve_array[i as usize] {
            for j in (i * i..end + 1).step_by(2 * i as usize) {
                sieve_array[j as usize] = false;
            }
        }
        i += 2;
    }
    let mut primes = Vec::new();
    for i in start..end + 1 {
        if sieve_array[i as usize] {
            primes.push(i);
        }
    }
    return primes;
}

fn calculate_bits(n: u64) -> u64 {
    let binary_string = format!("{:b}", n); // Convert n to binary string
    let sum_of_bits: u64 = binary_string
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum();
    n - sum_of_bits
}

fn fast_exponentiation(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exponent = exponent;
    while exponent > 0 {
        if exponent % 2 == 1 {
            result = (result * base) % modulus;
        }
        exponent = exponent >> 1;
        base = (base * base) % modulus;
    }
    return result;
}

/// Actual implementation of the factorial function using the prime swing algorithm

pub fn swing(n: u64, primes: Vec<u64>) -> u64 {
    let mut factors: Vec<u64> = Vec::new();
    let mut q: u64;
    for prime in primes {
        if prime > n {
            break;
        }
        q = n;
        while q > 0 {
            q /= prime;
            if q % 2 == 1 {
                factors.push(prime);
            }
        }
    }
    let product: u64 = factors.iter().product();
    return product;
}

// TODO: rewrite this part to avoid recursion.
pub fn odd_factorial(n: u64, primes: Vec<u64>) -> u64 {
    if n < 2 {
        return 1;
    } else {
        let half_factorial = odd_factorial(n / 2, primes.clone());
        let swing_factor = swing(n, primes);
        return fast_exponentiation(half_factorial, 2, u64::MAX) * swing_factor;
    }
}

pub fn eval(n: u64) -> u64 {
    if n < 10 {
        return (2..=n).product();
    } else {
        let bits = calculate_bits(n);
        let primes = sieve(2, n + 1);
        return fast_exponentiation(2 * odd_factorial(n, primes), bits, u64::MAX);
    }
}

pub fn factorial(n: u64) -> u64 {
    return eval(n);
}

// TODO : add more tests and performance tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sieve() {
        let result = sieve(1, 10);
        assert_eq!(result, vec![2, 3, 5, 7]);
    }

    #[test]
    fn test_factorial() {
        let result = factorial(5);
        assert_eq!(result, 120);
    }
}
