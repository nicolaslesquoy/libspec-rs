/// This module contains the implementation of the factorial function using the prime swing algorithm.
/// Copyright (c) Peter Luschny, 2000-2017. The original code is distributed under the CC-BY-SA 3.0 license. Please see the original code at `https://www.luschny.de/math/factorial/FastFactorialFunctions.htm` for more information.
/// The original code has been modified to fit the needs of this project by adding helper functions and implementing functions provided by external libraries in the original code.

/// Helper functions for prime number generation, bit calculation, and fast exponentiation

/// Sieve of Eratosthenes implementation to generate prime numbers between start and end
/// The function returns a vector of prime numbers between start and end
/// 
/// # Arguments
/// 
/// * `start` - The starting number of the range
/// * `end` - The ending number of the range
/// 
/// # Returns
/// 
/// * A vector of prime numbers between start and end
fn sieve(start: u64, end: u64) -> Vec<u64> {
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

/// Calculate the number of bits in the binary representation of a number
/// 
/// # Arguments
/// 
/// * `n` - The number whose bits are to be calculated
/// 
/// # Returns
/// 
/// * The number of bits in the binary representation of the number
fn calculate_bits(n: u64) -> u64 {
    let binary_string = format!("{:b}", n); // Convert n to binary string
    let sum_of_bits: u64 = binary_string
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)
        .sum();
    return n - sum_of_bits
}

/// Fast exponentiation algorithm to calculate the result of a^b mod m
/// 
/// # Arguments
/// 
/// * `base` - The base of the exponentiation
/// * `exponent` - The exponent of the exponentiation
/// * `modulus` - The modulus of the exponentiation
/// 
/// # Returns
/// 
/// * The result of the exponentiation a^b mod m
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

/// Actual implementation of the factorial function using the prime swing algorithm based on the SageMath implementation by Peter Luschny

// TODO : Add proper documentation for these intermediate functions

fn swing(n: u64, primes: &[u64]) -> u64 {
    let mut factors: Vec<u64> = Vec::new();
    let mut q: u64;
    for &prime in primes {
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

fn odd_factorial(n: u64, primes: &[u64]) -> u64 {
    if n < 2 {
        return 1;
    } else {
        return fast_exponentiation(odd_factorial(n / 2, primes), 2, u64::MAX) * swing(n, &primes);
    }
}

fn eval(n: u64) -> u64 {
    if n < 10 {
        return (2..=n).product();
    } else {
        let bits = calculate_bits(n);
        let primes = sieve(2, n + 1);
        return fast_exponentiation(2 * odd_factorial(n, &primes), bits, u64::MAX);
    }
}

/// Calculate the factorial of a number using the prime swing algorithm
/// 
/// # Arguments
/// 
/// * `n` - The number whose factorial is to be calculated
/// 
/// # Returns
/// 
/// * The factorial of the number
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
